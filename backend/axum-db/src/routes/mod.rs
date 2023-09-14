mod health_check;

use axum::response::IntoResponse;
pub use health_check::*;

pub async fn index() -> impl IntoResponse {
    "hello, world!"
}
