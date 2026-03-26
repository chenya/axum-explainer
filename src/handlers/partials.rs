use askama::Template;
use axum::{
    extract::{Path, Query, State},
    response::Response,
};
use std::collections::HashMap;

use crate::models::*;
use crate::state::AppState;
use crate::{error::AppError, handlers::render};
use std::sync::Arc;

// ── Layer detail (HTMX: click a stack layer) ─────────────────────────────────
#[derive(Template)]
#[template(path = "partials/layer_detail.html")]
pub struct LayerDetailTemplate {
    pub layer: Layer,
}

pub async fn layer_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    let layer = state
        .layers
        .iter()
        .find(|l| l.id == id.as_str())
        .cloned()
        .ok_or(AppError::NotFound)?;

    render(LayerDetailTemplate { layer })
}

#[derive(Template)]
#[template(path = "partials/flow_steps.html")]
pub struct FlowStepsTemplate {
    pub steps: Vec<FlowStep>,
    pub mode: String,
}

pub async fn flow_steps(
    State(state): State<Arc<AppState>>,
    Path(mode): Path<String>,
) -> Result<Response, AppError> {
    render(FlowStepsTemplate {
        steps: state.flow_steps.clone(),
        mode,
    })
}

// ── Handler sig builder (HTMX: extractor dropdowns) ──────────────────────────
#[derive(Template)]
#[template(path = "partials/handler_sig.html")]
pub struct HandlerSigTemplate {
    pub has_path: bool,
    pub has_tuple_path: bool,
    pub has_query: bool,
    pub has_body: bool,
}

pub async fn handler_sig(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let path_val = params.get("path").map(|s| s.as_str()).unwrap_or("none");
    let query_val = params.get("query").map(|s| s.as_str()).unwrap_or("none");
    let body_val = params.get("body").map(|s| s.as_str()).unwrap_or("none");

    render(HandlerSigTemplate {
        has_path: path_val != "none",
        has_tuple_path: path_val == "tuple",
        has_query: query_val != "none",
        has_body: body_val != "none",
    })
}
