use futures_util::{SinkExt, StreamExt};
use crate::api::{ApiHandlerState, AuthContext};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tracing::warn;
use uuid::Uuid;
use crate::api::errors::ApiError;
use crate::model::session::{MessageApiTask, MessageParticipantTask, MessageTaskParticipant, TypeMessage};
use crate::session::Session;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionInput {
    pub participant: Vec<Uuid>,
    pub filters: Vec<String>,
}

pub async fn start(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<ApiHandlerState>,
    Json(input): Json<SessionInput>,
) -> Result<Json<Uuid>, ApiError> {
    if input.participant.len() < 2 {
        return Err(ApiError::precondition_failed(
            "not enough users".to_owned(),
        ));
    }
    if !input.participant.contains(&auth_context.user){
        return Err(ApiError::precondition_failed(
            "owner must be among the participants".to_owned(),
        ));
    }

    let session_id = Uuid::new_v4();
    let session = Session::new(input.participant.clone(),input.filters,session_id);

    let mut sessions_lock = state.sessions.write().await;
    sessions_lock.insert(session_id, session);

    for user_id in input.participant {
        state.db.create_invitation(auth_context.user, user_id, session_id).await?;
    }

    Ok(Json(session_id))
}

pub async fn join(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<ApiHandlerState>,
    ws: WebSocketUpgrade,
    Path(session_id): Path<Uuid>
) -> Result<impl IntoResponse, ApiError> {
    let sessions_lock = state.sessions.read().await;
    if !sessions_lock.contains_key(&session_id) {
        return Err(ApiError::precondition_failed(
            "session does not exist".to_owned(),
        ));
    }

    let session = sessions_lock.get(&session_id).unwrap();

    if !session.participants.contains(&auth_context.user) {
        return Err(ApiError::precondition_failed(
            "user is not invited to the session".to_owned(),
        ));
    }

    let tx = session.tx.clone();
    let rx = session.rx.resubscribe();
    let user = auth_context.user;

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, tx, rx, user)))
}

async fn handle_socket(
    socket: WebSocket,
    session_tx: UnboundedSender<TypeMessage>,
    mut session_rx: broadcast::Receiver<MessageTaskParticipant>,
    user_id: Uuid
) {
    let (mut sender, mut receiver_ws) = socket.split();
    let (socket_tx, mut socket_rx) = unbounded_channel();

    tokio::spawn(async move {
        _ = session_tx.send(TypeMessage::Api(MessageApiTask::Join {user_id, tx:socket_tx}));
        loop {
            tokio::select! {
                Some(res) = receiver_ws.next() => {
                    match res {
                        Ok(msg) => {
                            match msg {
                                Message::Text(text) => {
                                    match serde_json::from_str::<MessageParticipantTask>(&text) {
                                        Ok(message) => {
                                            _ = session_tx.send(TypeMessage::Participant{user_id,message});
                                        },
                                        Err(error) => {
                                            warn!(?user_id,?error)
                                        }
                                    }
                                },
                                Message::Close(_) => {
                                    _ = session_tx.send(TypeMessage::Api(MessageApiTask::Leave(user_id)));
                                    return
                                }
                            _ => {}}
                        },
                        Err(_) => {
                            _ = session_tx.send(TypeMessage::Api(MessageApiTask::Leave(user_id)));
                            return
                        }
                    }
                },
                Some(msg) = socket_rx.recv() => {
                    if let Ok(msg) = serde_json::to_string(&msg) {
                        let message = Message::Text(Utf8Bytes::from(msg));
                        _ = sender.send(message).await;
                    }

                },
                Ok(msg) = session_rx.recv() => {
                    if let Ok(msg) = serde_json::to_string(&msg) {
                        let message = Message::Text(Utf8Bytes::from(msg));
                        _ = sender.send(message).await;
                    }
                },
            }
        }
    });
}

pub async fn invit(
    State(state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
){
    let result = state.db.get_invitations(auth_context.user).await;
    if let Ok(session_requests) = result {
        for session_request in session_requests {
            let mut sessions_lock = state.sessions.write().await;
            if let Some(session) = sessions_lock.clone().get(&session_request.session_id) {
                if !session.tx.is_closed() {
                    _ = state.db.delete_session_invitations(session_request.session_id).await;
                    sessions_lock.remove(&session.id);
                }
            }
        }
    }

}

pub fn session_router(api_handler: ApiHandlerState) -> Router {
    Router::new()
        .route("/", post(start))
        .route("/{id_session}", get(join))
        .route("/", get(invit))
        .with_state(api_handler)
}
