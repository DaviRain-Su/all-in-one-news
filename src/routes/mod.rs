mod health_check;
pub mod rebase;

use crate::templates::index::IndexTemplate;
use actix_web::HttpResponse;
use askama_actix::TemplateToResponse;
pub use health_check::*;

pub async fn index() -> HttpResponse {
    let template = IndexTemplate {};
    template.to_response()
}
