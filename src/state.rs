use crate::models::*;

// ── Shared application state ──────────────────────────────────────────────────
//
// This AppState is itself part of the tutorial: readers can see how
// we use Arc-backed data loaded once at startup and shared across handlers.

pub struct AppState {
    pub sections: Vec<Section>,
    pub layers: Vec<Layer>,
    pub routes: Vec<RouteRow>,
    pub extractors: Vec<ExtractorCard>,
    pub response_types: Vec<ResponseType>,
    pub flow_steps: Vec<FlowStep>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sections: all_sections(),
            layers: all_layers(),
            routes: all_routes(),
            extractors: all_extractors(),
            response_types: all_response_types(),
            flow_steps: all_flow_steps(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
