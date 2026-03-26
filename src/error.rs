use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    NotFound,
    TemplateRender(askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::TemplateRender(e) => {
                tracing::error!("template error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(e: askama::Error) -> Self {
        AppError::TemplateRender(e)
    }
}
