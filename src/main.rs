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
    HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(obj)
}

#[get("/echo/{name}")]
async fn echo(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.to_string())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // logging
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // API
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
