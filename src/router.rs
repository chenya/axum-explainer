use crate::handlers::{pages, partials};
use crate::state::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub fn build(state: Arc<AppState>) -> Router {
    // ── Router ────────────────────────────────────────────────────────────────
    //
    // Notice: this router IS the tutorial content — the structure here
    // mirrors exactly what section 2 (Router) explains.

    Router::new()
        // Full page routes (SSR via Askama)
        .route("/", get(pages::index))
        .route("/section/{slug}", get(pages::section))
        // HTMX partial routes (return HTML fragments, not full pages)
        .route("/htmx/layer/{id}", get(partials::layer_detail))
        .route("/htmx/flow/{mode}", get(partials::flow_steps))
        .route("/htmx/handler-sig", get(partials::handler_sig))
        // Static files (CSS, fonts)
        .nest_service("/static", ServeDir::new("static"))
        // Register AppState — accessible in every handler via State extractor
        .with_state(state)
        // Tower middleware layers
        .layer(TraceLayer::new_for_http())
}
