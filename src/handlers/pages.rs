use askama::Template;
use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::handlers::render;
use crate::models::*;
use crate::{AppState, error::AppError};

use std::sync::Arc;

// ── Shared nav context ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct NavCtx {
    pub prev_slug: Option<String>,
    pub prev_title: Option<String>,
    pub next_slug: Option<String>,
    pub next_title: Option<String>,
    pub current_index: usize, // 0-based
    pub total: usize,
    pub progress_pct: usize, // 0–100
}

impl NavCtx {
    pub fn for_slug(slug: &str) -> Self {
        let sections = all_sections();
        let total = sections.len();
        let idx = sections.iter().position(|s| s.slug == slug).unwrap_or(0);
        let prev = if idx > 0 {
            Some(&sections[idx - 1])
        } else {
            None
        };
        let next = if idx + 1 < total {
            Some(&sections[idx + 1])
        } else {
            None
        };
        Self {
            prev_slug: prev.map(|s| s.slug.clone()),
            prev_title: prev.map(|s| s.title.clone()),
            next_slug: next.map(|s| s.slug.clone()),
            next_title: next.map(|s| s.title.clone()),
            current_index: idx,
            total,
            progress_pct: ((idx + 1) * 100) / total,
        }
    }
}

// ── Templates ─────────────────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub sections: Vec<Section>,
    pub layers: Vec<Layer>,
    pub current: String,
}

#[derive(Template)]
#[template(path = "section_stack.html")]
pub struct SectionStackTemplate {
    pub sections: Vec<Section>,
    pub layers: Vec<Layer>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_router.html")]
pub struct SectionRouterTemplate {
    pub sections: Vec<Section>,
    pub routes: Vec<RouteRow>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_handlers.html")]
pub struct SectionHandlersTemplate {
    pub sections: Vec<Section>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_extractors.html")]
pub struct SectionExtractorsTemplate {
    pub sections: Vec<Section>,
    pub extractors: Vec<ExtractorCard>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_responses.html")]
pub struct SectionResponsesTemplate {
    pub sections: Vec<Section>,
    pub response_types: Vec<ResponseType>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_state.html")]
pub struct SectionStateTemplate {
    pub sections: Vec<Section>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_middleware.html")]
pub struct SectionMiddlewareTemplate {
    pub sections: Vec<Section>,
    pub current: String,
    pub nav: NavCtx,
}

#[derive(Template)]
#[template(path = "section_flow.html")]
pub struct SectionFlowTemplate {
    pub sections: Vec<Section>,
    pub flow_steps: Vec<FlowStep>,
    pub current: String,
    pub nav: NavCtx,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

pub async fn index(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    render(IndexTemplate {
        sections: state.sections.clone(),
        layers: state.layers.clone(),
        current: "home".into(),
    })
}

enum SectionPage {
    Stack,
    Router,
    Handlers,
    Extractors,
    Responses,
    State,
    Middleware,
    Flow,
}

impl SectionPage {
    // slug → variant: one place to add/rename a section
    fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "stack" => Some(Self::Stack),
            "router" => Some(Self::Router),
            "handlers" => Some(Self::Handlers),
            "extractors" => Some(Self::Extractors),
            "responses" => Some(Self::Responses),
            "state" => Some(Self::State),
            "middleware" => Some(Self::Middleware),
            "flow" => Some(Self::Flow),
            _ => None,
        }
    }
    // variant → rendered response
    fn render(self, state: &AppState, slug: String) -> Result<Response, AppError> {
        match self {
            Self::Stack => render(SectionStackTemplate {
                sections: state.sections.clone(),
                layers: state.layers.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Router => render(SectionRouterTemplate {
                sections: state.sections.clone(),
                routes: state.routes.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Handlers => render(SectionHandlersTemplate {
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Extractors => render(SectionExtractorsTemplate {
                extractors: state.extractors.clone(),
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Responses => render(SectionResponsesTemplate {
                response_types: state.response_types.clone(),
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::State => render(SectionStateTemplate {
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Middleware => render(SectionMiddlewareTemplate {
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
            Self::Flow => render(SectionFlowTemplate {
                flow_steps: state.flow_steps.clone(),
                sections: state.sections.clone(),
                nav: NavCtx::for_slug(&slug),
                current: slug,
            }),
        }
    }
}

// The handler is now dead simple — no section-specific knowledge
pub async fn section(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    match SectionPage::from_slug(&slug) {
        Some(page) => page.render(&state, slug),
        None => Err(AppError::NotFound),
    }
}
