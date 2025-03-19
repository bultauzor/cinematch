use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::error;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    INTERNAL_SERVER_ERROR,
    UNAUTHORIZED,
    BAD_REQUEST,
}

impl From<ErrorCode> for StatusCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::INTERNAL_SERVER_ERROR => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::UNAUTHORIZED => StatusCode::UNAUTHORIZED,
            ErrorCode::BAD_REQUEST => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub error: String,
    pub error_code: ErrorCode,
}

impl From<ApiError> for ApiErrorBody {
    fn from(value: ApiError) -> Self {
        ApiErrorBody {
            error: value.error,
            error_code: value.error_code,
        }
    }
}

pub struct ApiError {
    pub error: String,
    pub error_code: ErrorCode,
}

impl ApiError {
    pub fn internal(internal_msg: &str) -> Self {
        error!(internal_msg);
        ApiError {
            error: "Internal server error".to_string(),
            error_code: ErrorCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn unauthorized() -> Self {
        ApiError {
            error: "Unauthorized".to_string(),
            error_code: ErrorCode::UNAUTHORIZED,
        }
    }

    pub fn bad_request(error: String) -> Self {
        ApiError {
            error,
            error_code: ErrorCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status: StatusCode = self.error_code.into();
        let body: ApiErrorBody = self.into();
        (status, Json(body)).into_response()
    }
}
