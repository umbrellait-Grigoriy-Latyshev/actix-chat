use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TTT {
    sum: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
    let obj = TTT { sum: 10 };
    HttpResponse::Ok().status(StatusCode::OK).json(obj)
}

#[get("/echo/{name}")]
async fn echo(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // logging
            .wrap(Logger::new("%a %{User-Agent}i %r %s, %T secs"))
            // API
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
