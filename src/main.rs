mod config;
mod error;
mod handlers;
mod models;
mod router;
mod state;

use state::AppState;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // Structured logging with tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_explainer=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();
    let state = Arc::new(AppState::new());
    let app = router::build(state);

    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to port {}", config.bind_addr.port()));

    info!("Axum Explainer listening on http://{}", config.bind_addr);
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server Error"));
}
