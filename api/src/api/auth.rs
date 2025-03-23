use std::ops::Deref;

use super::errors::ApiError;
use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub avatar: Option<String>,
}

pub async fn register(
    auth_api_url: State<String>,
    Json(input): Json<UserInput>,
) -> Result<Json<Token>, ApiError> {
    if input.username.len() < 2 {
        return Err(ApiError::precondition_failed(
            "username is too short".to_string(),
        ));
    }

    if input.username.len() > 32 {
        return Err(ApiError::precondition_failed(
            "username is too long".to_string(),
        ));
    }

    if input.password.len() < 4 {
        return Err(ApiError::precondition_failed(
            "password is too short".to_string(),
        ));
    }

    if input.password.len() > 128 {
        return Err(ApiError::precondition_failed(
            "password is too long".to_string(),
        ));
    }

    let response = reqwest::Client::new()
        .post(format!("{}/register", auth_api_url.deref()))
        .json(&input)
        .send()
        .await
        .map_err(|_| ApiError::internal("unable to make a request to auth api"))?;

    let status = response.status();
    if status.is_success() {
        let token: Token = response
            .json()
            .await
            .map_err(|_| ApiError::internal("unable to parse auth api token"))?;

        Ok(Json(token))
    } else if status == reqwest::StatusCode::BAD_REQUEST {
        Err(ApiError::bad_request(
            response.text().await.unwrap_or("Bad request".to_string()),
        ))
    } else {
        Err(ApiError::internal("auth api is having a seizure..."))
    }
}

pub async fn auth(
    auth_api_url: State<String>,
    Json(input): Json<InputCredentials>,
) -> Result<Json<Token>, ApiError> {
    let response = reqwest::Client::new()
        .post(format!("{}/auth", auth_api_url.deref()))
        .json(&input)
        .send()
        .await
        .map_err(|_| ApiError::internal("unable to make a request to auth api"))?;

    let status = response.status();
    if status.is_success() {
        let token: Token = response
            .json()
            .await
            .map_err(|_| ApiError::internal("unable to parse auth api token"))?;

        Ok(Json(token))
    } else if status == reqwest::StatusCode::BAD_REQUEST {
        Err(ApiError::bad_request(
            response.text().await.unwrap_or("Bad request".to_string()),
        ))
    } else if status == reqwest::StatusCode::UNAUTHORIZED {
        Err(ApiError::unauthorized())
    } else {
        Err(ApiError::internal("auth api is having a seizure..."))
    }
}

pub fn auth_router(auth_api_url: String) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/auth", post(auth))
        .with_state(auth_api_url)
}
