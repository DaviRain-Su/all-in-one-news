mod greet;
mod health_check;

pub use greet::*;
pub use health_check::*;

use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

pub async fn index() -> impl IntoResponse {
    let template = HelloTemplate::default();
    HtmlTemplate(template)
}

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct HelloTemplate {
    pub name: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
