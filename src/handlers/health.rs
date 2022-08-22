use actix_web::{get, http::StatusCode, HttpResponse, Responder};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().status(StatusCode::OK).body("healthy")
}
