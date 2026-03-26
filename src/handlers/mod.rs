pub mod pages;
pub mod partials;

use crate::error::AppError;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub fn render<T: Template>(tmpl: T) -> Result<Response, AppError> {
    Ok(Html(tmpl.render()?).into_response())
}
