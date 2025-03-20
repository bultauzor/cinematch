pub mod api;
pub mod db;

use crate::api::ApiHandler;
use crate::api::ApiHandlerState;
use crate::db::DbHandler;

use std::str::FromStr;

use tracing::{error, info};

/// Get env var as string or panic
pub fn env_get(env: &'static str) -> String {
    let env_panic = |e| {
        error!("{env} is not set ({})", e);
        std::process::exit(1);
    };

    std::env::var(env).map_err(env_panic).unwrap()
}

/// Get env var as string or panic, with a default string
pub fn env_get_or(env: &'static str, other: String) -> String {
    std::env::var(env).unwrap_or(other)
}

/// Get env var as number or panic, with a default number
pub fn env_get_num_or<T: FromStr>(env: &'static str, other: T) -> T {
    let env_parse_panic = |v| {
        error!("can't parse {env} ({v})");
        std::process::exit(1);
    };
    match std::env::var(env) {
        Ok(v) => v.parse::<T>().map_err(|_| env_parse_panic(v)).unwrap(),
        Err(_) => other,
    }
}

/// Start logger
#[inline]
pub fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_env_var("LOG_LEVEL")
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

#[tokio::main]
async fn main() {
    let address = env_get_or("ADDRESS", "0.0.0.0".to_string());
    let port: u16 = env_get_num_or("PORT", 8080);
    let postgresql_uri = env_get("POSTGRESQL_ADDON_URI");
    let mut auth_api_url = env_get("AUTH_API_URL");

    // ops friendlyness
    if auth_api_url.ends_with("/") {
        auth_api_url.pop();
    }

    init_logger();

    info!("Connecting to database");
    let db = match DbHandler::connect(&postgresql_uri).await {
        Some(db) => {
            info!("Connected to database!");
            db
        }
        None => {
            error!("Failed to connect to database");
            std::process::exit(1);
        }
    };

    let app = api::app(ApiHandlerState::new(ApiHandler { db }), auth_api_url);

    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}"))
        .await
        .unwrap();

    info!("Starting api at http://{address}:{port}");
    _ = axum::serve(listener, app).await
}
