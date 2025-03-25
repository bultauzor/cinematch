use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use uuid::Uuid;

use crate::model::seen::{SeenContent, SeenContentInput, SeenGradeInput};

use super::{ApiHandlerState, AuthContext, errors::ApiError};

pub fn seen_router(api_handler: ApiHandlerState) -> Router {
    Router::new()
        .route("/me", get(get_seen_content))
        .route("/me", post(post_seen_content))
        .route("/me/{content}", delete(delete_seen_content))
        .route("/me/{content}/grade", patch(patch_seen_content_grade))
        .route("/me/{content}/grade", delete(delete_seen_content_grade))
        .with_state(api_handler)
}

pub fn check_grade(grade: f64) -> Result<(), ApiError> {
    if grade.is_sign_negative() {
        return Err(ApiError::precondition_failed(
            "grade cannot be negative".to_owned(),
        ));
    }

    if grade > 10.0 {
        return Err(ApiError::precondition_failed(
            "grade cannot be supperior to 10".to_owned(),
        ));
    }

    Ok(())
}

pub async fn get_seen_content(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
) -> Result<Json<Vec<SeenContent>>, ApiError> {
    match api_handler_state
        .db
        .get_seen_content_by_user_id(&auth_context.user)
        .await
    {
        Ok(content) => Ok(Json(content)),
        Err(err) => Err(ApiError::from(err)),
    }
}

pub async fn post_seen_content(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Json(input): Json<SeenContentInput>,
) -> Result<StatusCode, ApiError> {
    if let Some(grade) = input.grade {
        check_grade(grade)?;
    }

    let content_exist = api_handler_state
        .db
        .content_exist(input.content_id)
        .await
        .map_err(ApiError::from)?;

    if !content_exist {
        Err(ApiError::bad_request(format!(
            "content {} does not exist",
            input.content_id
        )))
    } else {
        api_handler_state
            .db
            .insert_seen_content(input, &auth_context.user)
            .await
            .map(|_| StatusCode::CREATED)
            .map_err(ApiError::from)
    }
}

pub async fn delete_seen_content(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Path(content_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Uuid>), ApiError> {
    match api_handler_state
        .db
        .delete_seen_content(&content_id, &auth_context.user)
        .await
    {
        Ok(content) => Ok((StatusCode::NO_CONTENT, Json(content))),
        Err(err) => Err(ApiError::from(err)),
    }
}

pub async fn patch_seen_content_grade(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Path(content_id): Path<Uuid>,
    Json(input): Json<SeenGradeInput>,
) -> Result<(), ApiError> {
    if let Some(grade) = input.grade {
        check_grade(grade)?;
    }

    match api_handler_state
        .db
        .update_seen_content_grade(input.grade, &content_id, &auth_context.user)
        .await
    {
        Ok(()) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(ApiError::bad_request(
            "you have not seen this content".to_string(),
        )),
        Err(e) => Err(ApiError::from(e)),
    }
}

pub async fn delete_seen_content_grade(
    Extension(auth_context): Extension<AuthContext>,
    State(api_handler_state): State<ApiHandlerState>,
    Path(content_id): Path<Uuid>,
) -> Result<(), ApiError> {
    match api_handler_state
        .db
        .update_seen_content_grade(None, &content_id, &auth_context.user)
        .await
    {
        Ok(()) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(ApiError::bad_request(
            "you have not seen this content".to_string(),
        )),
        Err(e) => Err(ApiError::from(e)),
    }
}
