use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{Path, State},
    http::Response,
    routing::{get, post},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbHandler;

use super::{ApiHandlerState, AuthContext, errors::ApiError};

#[derive(Deserialize)]
pub struct UploadImageRequest {
    pub image: String,
}

pub fn avatar_router(api_handler: ApiHandlerState) -> Router {
    Router::new()
        .route("/", post(upload_avatar))
        .route("/", get(get_avatar))
        .with_state(api_handler)
}

pub fn avatar_router_unauth(api_handler: ApiHandlerState) -> Router {
    Router::new()
        .route("/{user_id}", get(get_avatar_by_user))
        .with_state(api_handler)
}

pub async fn upload_avatar(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Json(image): Json<UploadImageRequest>,
) -> Result<(), ApiError> {
    api_handler_state
        .db
        .update_avatar(&auth_context.user, image.image)
        .await
        .map_err(ApiError::from)
}

pub async fn get_avatar(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
) -> Result<Response<Body>, ApiError> {
    get_avatar_handler(auth_context.user, &api_handler_state.db).await
}

pub async fn get_avatar_by_user(
    State(api_handler_state): State<ApiHandlerState>,
    Path(user_id): Path<Uuid>,
) -> Result<Response<Body>, ApiError> {
    get_avatar_handler(user_id, &api_handler_state.db).await
}

pub async fn get_avatar_handler(user_id: Uuid, db: &DbHandler) -> Result<Response<Body>, ApiError> {
    let base64_image = db.get_avatar(user_id).await.map_err(ApiError::from)?;

    if let None = base64_image {
        return Err(ApiError::not_found("avatar not found".to_string()));
    }

    Ok(Response::builder()
        .header("Content-Type", "text/plain")
        .body(base64_image.unwrap().into())
        .unwrap())
}
