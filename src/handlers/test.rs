use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};

use crate::structs::test::TTT;

#[get("/")]
pub async fn hello() -> impl Responder {
    let obj = TTT::new(10);
    HttpResponse::Ok().status(StatusCode::OK).json(obj)
}

#[get("/echo/{name}")]
pub async fn echo(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.to_string())
}
