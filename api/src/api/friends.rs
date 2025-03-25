use std::collections::HashMap;
use crate::api::{ApiHandlerState, AuthContext};
use crate::api::errors::ApiError;
use axum::extract::{Query, State};
use axum::routing::{post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct FriendRequest {
    pub user_id: Uuid,
    pub friend_id: Uuid,
}


#[derive(Serialize)]
pub struct Friend{
    pub user_id: Uuid,
    pub friend_id: Uuid
}

#[derive(Deserialize)]
pub struct FriendInput{
    pub user_id: Uuid,
    pub friend_username: String
}

// #[axum::debug_handler]
pub async fn accept_friend(
    State(api_handler_state): State<ApiHandlerState>,
    Extension(auth_context): Extension<AuthContext>,
    Query(query): Query<HashMap<String, String>>,
) ->Result<Json<Friend>, ApiError> {
    let _query = query
        .get("query")
        .ok_or(ApiError::bad_request("No `query` parameter".into()))?;

    let friend_username = query
        .get("friend_username")
        .ok_or(ApiError::bad_request("Missing `friend_username` parameter".into()))?;

    let friend_input = FriendInput {
        user_id: auth_context.user,
        friend_username: friend_username.to_string(),
    };


    let res = api_handler_state.db.friend_accept(&friend_input).await?;

    Ok(Json(res))
}

pub async fn request_friend(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Query(query): Query<HashMap<String, String>>
) -> Result<Json<FriendRequest>, ApiError> {
    let _query = &query
    .get("query")
    .ok_or(ApiError::bad_request("No `query` parameter".into()))?;

    let &friend_username = &query
        .get("friend_username")
        .ok_or(ApiError::bad_request("Missing `friend_username` parameter".into()))?;

    let friend_input = FriendInput {
        user_id: auth_context.user,
        friend_username: friend_username.to_string(),
    };


    let res= api_handler_state.db.friend_request(&friend_input).await?;

    Ok(Json(res))
}

pub fn friends_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/accept", post(accept_friend))
        .route("/request", post(request_friend))
        .with_state(api_handler_state)
}