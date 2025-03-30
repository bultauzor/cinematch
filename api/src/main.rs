pub mod api;
pub mod db;
pub mod model;
pub mod provider;

use crate::api::ApiHandler;
use crate::api::ApiHandlerState;
use crate::db::DbHandler;
use crate::provider::tmdb::TmdbProvider;

use biscuit_auth::PublicKey;
use opentelemetry::global::set_tracer_provider;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::OnceLock;
use tracing::{debug, error, info};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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

/// Get env var and parse it or panic, with a default
pub fn env_get_parse_or<T: FromStr>(env: &'static str, other: T) -> T {
    std::env::var(env)
        .ok()
        .and_then(|e| e.parse::<T>().ok())
        .unwrap_or(other)
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

static RESOURCE: OnceLock<Resource> = OnceLock::new();

#[inline]
fn get_resource<'a>() -> &'a Resource {
    RESOURCE.get_or_init(|| {
        Resource::builder()
            .with_service_name("cinematch-api")
            .build()
    })
}

pub fn init_otlp_tracing(endpoint: String) -> SdkTracerProvider {
    let exporter = if endpoint.is_empty() {
        SpanExporter::builder()
            .with_tonic()
            .build()
            .expect("failed to create log exporter")
    } else {
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()
            .expect("failed to create log exporter")
    };

    SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource().clone())
        .build()
}

#[derive(Default)]
pub struct TracingGuard {
    _tracing_guard: Option<SdkTracerProvider>,
}

/// Start tracing
#[inline]
pub fn init_tracing(otlp_endpoint: Option<String>) -> TracingGuard {
    let mut guard = TracingGuard::default();

    let filter = || {
        tracing_subscriber::EnvFilter::builder()
            .with_env_var("LOG_LEVEL")
            .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
            .from_env_lossy()
    };

    match otlp_endpoint {
        None => {
            tracing_subscriber::fmt().with_env_filter(filter()).init();
        }
        Some(endpoint) => {
            let tracer_provider = init_otlp_tracing(endpoint);
            set_tracer_provider(tracer_provider.clone());

            let tracer = tracer_provider.tracer("cinematch-api");

            let otel_layer = tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(filter());

            let fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter());

            tracing_subscriber::registry()
                .with(otel_layer)
                .with(fmt_layer)
                .init();
            guard._tracing_guard = Some(tracer_provider);
            debug!("otlp tracing activated")
        }
    }

    guard
}

#[tokio::main]
async fn main() {
    let otlp_enpoint = std::env::var("OTEL_EXPORTER_ENDPOINT").ok();

    let guard = init_tracing(otlp_enpoint);

    let address = env_get_parse_or("ADDRESS", IpAddr::from([0, 0, 0, 0]));
    let port: u16 = env_get_num_or("PORT", 8080);
    let postgresql_uri = env_get("POSTGRESQL_ADDON_URI");
    let mut auth_api_url = env_get("AUTH_API_URL");
    let tmdb_token = env_get("TMDB_TOKEN");
    let biscuit_pubkey = match PublicKey::from_bytes_hex(&env_get("PUBLIC_KEY")) {
        Ok(key) => key,
        Err(e) => {
            error!("can not parse biscuit public key : {e}",);
            std::process::exit(1);
        }
    };

    // ops friendlyness
    if auth_api_url.ends_with("/") {
        auth_api_url.pop();
    }

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

    let tmdb_provider = match TmdbProvider::new(&tmdb_token) {
        Some(tmdb_provider) => tmdb_provider,
        None => {
            error!("Failed to initialize tmdb provider");
            std::process::exit(1);
        }
    };

    let app = api::app(
        ApiHandlerState::new(ApiHandler {
            db,
            provider: tmdb_provider,
        }),
        auth_api_url,
        biscuit_pubkey,
    );

    let socket_addr = SocketAddr::new(address, port);
    let listener = tokio::net::TcpListener::bind(&socket_addr)
        .await
        .expect("server should start");

    match address {
        IpAddr::V4(address) => info!("Starting api at http://{address}:{port}"),
        IpAddr::V6(address) => info!("Starting api at http://[{address}]:{port}"),
    }
    _ = axum::serve(listener, app).await;

    drop(guard);
}
