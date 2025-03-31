use crate::api::errors::ApiError;
use crate::api::{ApiHandlerState, AuthContext};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct FriendRequest {
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub user_username: String,
    pub user_avatar: Option<String>,
}

#[derive(Serialize)]
pub struct Friend {
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub friend_username: String,
}

#[derive(Deserialize)]
pub struct FriendInput {
    pub user_id: Uuid,
    pub friend_username: String,
}

pub async fn delete_friend(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(user_id): Path<Uuid>,
) -> Result<(), ApiError> {
    api_handler_state
        .db
        .delete_friend(auth_context.user, user_id)
        .await?;
    Ok(())
}

pub async fn get_all_friends(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
) -> Result<Json<Vec<Friend>>, ApiError> {
    let res = api_handler_state
        .db
        .get_all_friends(auth_context.user)
        .await?;

    Ok(Json(res))
}

pub async fn create_invitation(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Json(input): Json<String>,
) -> Result<(), ApiError> {
    api_handler_state
        .db
        .create_invitation_friend(auth_context.user, input.to_string())
        .await?;

    Ok(())
}

pub fn friends_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/", get(get_all_friends))
        .route("/", post(create_invitation))
        .route("/{user_id}", post(delete_friend))
        .with_state(api_handler_state)
}
