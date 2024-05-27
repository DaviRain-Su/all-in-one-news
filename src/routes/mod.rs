mod health_check;
pub mod rebase;

use actix_web::HttpResponse;
pub use health_check::*;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
