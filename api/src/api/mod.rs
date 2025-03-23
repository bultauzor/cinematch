pub mod auth;
pub mod errors;
pub mod search;

use axum::Router;
use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, Response, header};
use axum::middleware::Next;
use axum::routing::get;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tracing::info;

use crate::db::DbHandler;
use crate::provider::tmdb::TmdbProvider;

pub struct ApiHandler {
    pub db: DbHandler,
    pub provider: TmdbProvider,
}

#[derive(Clone)]
pub struct ApiHandlerState(Arc<ApiHandler>);

impl ApiHandlerState {
    pub fn new(handler: ApiHandler) -> Self {
        Self(Arc::new(handler))
    }
}

impl Deref for ApiHandlerState {
    type Target = Arc<ApiHandler>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ApiHandlerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn app(api_handler: ApiHandlerState, auth_api_url: String) -> Router<()> {
    Router::new()
        .route("/ping", get(ping))
        .route("/teapot", get(teapot))
        .merge(self::auth::auth_router(auth_api_url))
        .merge(search::search_router(api_handler))
        .layer(axum::middleware::from_fn(log_middleware))
        .layer(axum::middleware::from_fn(cors_middleware))
}

pub async fn log_middleware(request: Request, next: Next) -> Response<Body> {
    let path = request.uri().path().to_string();

    info!("{} {}", request.method(), &path);

    next.run(request).await
}

pub async fn cors_middleware(request: Request, next: Next) -> Response<Body> {
    let method = request.method().clone();

    let mut response = next.run(request).await;

    if let Method::OPTIONS = method {
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET,POST,PUT,DELETE,OPTIONS"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_MAX_AGE,
            // 24h in seconds
            HeaderValue::from_static("86400"),
        );

        *response.status_mut() = axum::http::StatusCode::OK
    }

    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );

    response
}

pub async fn ping() -> &'static str {
    "PONG !"
}

pub async fn teapot() -> Response<Body> {
    Response::builder()
        .status(418)
        .body(Body::new("I am a teapot !".to_string()))
        .unwrap()
}
