use axum::Router;
use axum::body::Body;
use axum::extract::Request;
use axum::http::Response;
use axum::middleware::Next;
use axum::routing::get;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::db::DbHandler;

pub struct ApiHandler {
    pub db: DbHandler,
}

#[derive(Clone)]
pub struct ApiHandlerState(Arc<Mutex<ApiHandler>>);

impl ApiHandlerState {
    pub fn new(handler: ApiHandler) -> Self {
        Self(Arc::new(Mutex::new(handler)))
    }
}

impl Deref for ApiHandlerState {
    type Target = Arc<Mutex<ApiHandler>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ApiHandlerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn app(_api_handler: ApiHandlerState) -> Router<()> {
    Router::new()
        .route("/ping", get(ping))
        .route("/teapot", get(teapot))
        .layer(axum::middleware::from_fn(log_middleware))
}

pub async fn log_middleware(request: Request, next: Next) -> Response<Body> {
    let path = request.uri().path().to_string();

    info!("{} {}", request.method(), &path);

    next.run(request).await
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
