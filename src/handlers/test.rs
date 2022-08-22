use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Test {
    id: u32,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    let obj = Test { id: 1 };
    HttpResponse::Ok().status(StatusCode::OK).json(obj)
}

#[get("/echo/{name}")]
pub async fn echo(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.to_string())
}
