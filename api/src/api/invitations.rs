use crate::api::errors::ApiError;
use crate::api::friends::FriendRequest;
use crate::api::{ApiHandlerState, AuthContext};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use uuid::Uuid;

pub async fn get_invitations(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
) -> Result<Json<Vec<FriendRequest>>, ApiError> {
    let res = api_handler_state
        .db
        .get_invitations_friends(auth_context.user)
        .await?;

    Ok(Json(res))
}

pub async fn accept_invitation(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    api_handler_state
        .db
        .accept_invitation(auth_context.user, user_id)
        .await?;
    Ok(())
}

pub async fn refuse_invitation(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    api_handler_state
        .db
        .refuse_invitation(auth_context.user, user_id)
        .await?;

    Ok(())
}

pub fn invitations_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/", get(get_invitations))
        .route("/{user_id}/accept", post(accept_invitation))
        .route("/{user_id}/refuse", post(refuse_invitation))
        .with_state(api_handler_state)
}
