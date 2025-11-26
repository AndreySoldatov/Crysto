use axum::Router;
use sqlx::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::EnvConfig;

mod auth;
mod config;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}

#[derive(Clone)]
struct AppState {
    dbpool: PgPool,
    config: EnvConfig,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    let env_config = envy::from_env::<EnvConfig>().unwrap();

    let dbpool = PgPool::connect(&env_config.database_url).await;
    if let Err(e) = dbpool {
        error!(db_url = %env_config.database_url, error = %e, "Database connection error!");
        return;
    }
    let appstate = AppState {
        dbpool: dbpool.unwrap(),
        config: env_config.clone(),
    };
    info!(db_url = %env_config.database_url, "Database connected");

    let router = Router::new()
        .nest("/auth", auth::auth_router())
        .fallback(|| async { "Hello from Crysto API!" })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG)),
        )
        .with_state(appstate);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", env_config.host, env_config.port))
            .await
            .unwrap();
    info!(host = %env_config.host, port = %env_config.port, "Starting listening");

    axum::serve(listener, router).await.unwrap();
}
