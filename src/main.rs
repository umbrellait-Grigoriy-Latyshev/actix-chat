use actix_web::{App, HttpServer};

use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;

mod handlers;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = env::var("HOST").expect("HOST variable not set!");
    let port = env::var("PORT")
        .expect("PORT variable not set!")
        .parse::<u16>()
        .expect("Cannot parse PORT as integer!");

    HttpServer::new(|| {
        App::new()
            // logging
            .wrap(Logger::new("%a %{User-Agent}i %r %s, %T secs"))
            // API
            .service(handlers::test::hello)
            .service(handlers::test::echo)
    })
    .bind((host, port))
    .expect("host/addr already in use!")
    .run()
    .await
}
