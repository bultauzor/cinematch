pub mod auth;
pub mod avatar;
pub mod content;
pub mod errors;
pub mod friends;
pub mod invitations;
pub mod search;
pub mod seen;
mod session;
pub mod recommendation;

use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, Response, header};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use biscuit_auth::{Authorizer, Biscuit, PublicKey};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

use crate::db::DbHandler;
use crate::provider::tmdb::TmdbProvider;
use crate::recommender::cinematch::CinematchRecommender;
use crate::session::Session;

pub struct ApiHandler {
    pub db: DbHandler,
    pub provider: Arc<TmdbProvider>,
    pub recommender: Arc<CinematchRecommender>,
    pub sessions: Arc<RwLock<HashMap<Uuid, Arc<Session>>>>,
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

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user: Uuid,
    pub username: String,
}

pub fn app(
    api_handler: ApiHandlerState,
    auth_api_url: String,
    public_key: PublicKey,
) -> Router<()> {
    Router::new()
        .route("/ping", get(ping))
        .route("/teapot", get(teapot))
        .nest("/avatar", avatar::avatar_router_unauth(api_handler.clone()))
        .merge(self::auth::auth_router(auth_api_url))
        .merge(api(api_handler, public_key))
        .layer(axum::middleware::from_fn(log_middleware))
        .layer(axum::middleware::from_fn(cors_middleware))
}

pub fn api(api_handler: ApiHandlerState, public_key: PublicKey) -> Router<()> {
    Router::new()
        .route("/auth_ping", get(auth_ping))
        .merge(search::search_router(api_handler.clone()))
        .nest("/avatar", avatar::avatar_router(api_handler.clone()))
        .nest("/recommendation", recommendation::recommendation_router(api_handler.clone()))
        .nest("/seen", seen::seen_router(api_handler.clone()))
        .nest("/session", session::session_router(api_handler.clone()))
        .nest("/friends", friends::friends_router(api_handler.clone()))
        .nest(
            "/invitations",
            invitations::invitations_router(api_handler.clone()),
        )
        .nest("/content", content::content_router(api_handler.clone()))
        .layer(axum::middleware::from_fn(move |req, next| {
            auth_middleware(req, next, public_key)
        }))
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
            HeaderValue::from_static("GET,POST,PUT,DELETE,OPTIONS,PATCH"),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*,Authorization"),
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

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
    public_key: PublicKey,
) -> Response<Body> {
    fn token_extract(header: &str, request: &mut Request) -> Option<String> {
        request
            .headers()
            .get(header)
            .and_then(|e| e.to_str().ok())
            .and_then(|authorization| {
                authorization
                    .to_string()
                    .strip_prefix("Bearer ")
                    .map(ToString::to_string)
                    .or(urlencoding::decode(authorization)
                        .map(|e| e.to_string())
                        .ok())
            })
    }

    let token = match token_extract("Authorization", &mut request) {
        Some(t) => t,
        None => match token_extract("Sec-WebSocket-Protocol", &mut request) {
            Some(t) => t,
            None => return self::errors::ApiError::unauthorized().into_response(),
        },
    };

    let biscuit = match Biscuit::from_base64(token, public_key) {
        Ok(token) => token,
        Err(_) => return self::errors::ApiError::unauthorized().into_response(),
    };

    let mut authorizer = Authorizer::new();
    match authorizer
        .add_code(r#"allow if user($u);"#)
        .and_then(|_| authorizer.add_token(&biscuit))
        .and_then(|_| authorizer.authorize())
    {
        Ok(a) => a,
        Err(_) => return self::errors::ApiError::unauthorized().into_response(),
    };

    let (user, username) = authorizer
        .query::<&str, (String, String), _>(
            "data($id, $username) <- user($id), username($username)",
        )
        .unwrap()
        .pop()
        .unwrap();

    let user = Uuid::parse_str(&user).unwrap();

    let auth_context = AuthContext { user, username };
    request.extensions_mut().insert(auth_context);

    next.run(request).await
}

pub async fn ping() -> &'static str {
    "PONG !"
}

pub async fn auth_ping(Extension(auth_context): Extension<AuthContext>) -> &'static str {
    debug!("{auth_context:#?}");
    "PONG !"
}

pub async fn teapot() -> Response<Body> {
    Response::builder()
        .status(418)
        .body(Body::new("I am a teapot !".to_string()))
        .unwrap()
}
