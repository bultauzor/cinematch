use crate::api::errors::ApiError;
use crate::api::ApiHandlerState;
use crate::model::content::ContentView;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

pub fn content_router(api_handler_state: ApiHandlerState) -> Router {
    Router::new()
        .route("/{content_id}", get(get_content_by_id))
        .route("/genres", get(get_genres))
        .with_state(api_handler_state)
}

pub async fn get_content_by_id(
    State(api_handler_state): State<ApiHandlerState>,
    Path(content_id): Path<Uuid>,
) -> Result<Json<ContentView>, ApiError> {
    let res = api_handler_state
        .db
        .get_content_by_id(&content_id)
        .await?
        .map(Into::into)
        .ok_or(ApiError::not_found(format!(
            "Content with id {} not found",
            content_id
        )))?;

    Ok(Json(res))
}

pub async fn get_genres(
    State(api_handler_state): State<ApiHandlerState>,
) -> Result<Json<Vec<String>>, ApiError> {
    let res = api_handler_state.db.get_genres().await?;

    Ok(Json(res))
}
