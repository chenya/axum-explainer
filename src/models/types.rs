use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: usize,
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: &'static str,
    pub name: &'static str,
    pub badge: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub detail: &'static str,
    pub color_class: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowStep {
    pub delay_ms: usize,
    pub num: usize,
    pub label: &'static str,
    pub description: &'static str,
    pub layer: &'static str, // "tokio" | "hyper" | "tower" | "axum" | "handler"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRow {
    pub method: &'static str,
    pub pattern: &'static str,
    pub handler: &'static str,
    pub example: &'static str,
    pub highlight: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorCard {
    pub name: &'static str,
    pub type_sig: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub code: &'static str,
    pub color: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseType {
    pub type_name: &'static str,
    pub description: &'static str,
    pub http_info: &'static str,
    pub color: &'static str,
}
