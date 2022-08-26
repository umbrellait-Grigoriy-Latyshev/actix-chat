use actix_web::{get, http::StatusCode, HttpResponse, Responder, Scope};

use super::scope;

pub fn api() -> Scope {
    scope(None).service(health)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().status(StatusCode::OK).body("healthy")
}
