use actix_web::{App, HttpServer};

use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;

mod handlers;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // logging
            .wrap(Logger::new("%a %{User-Agent}i %r %s, %T secs"))
            // API
            .service(handlers::test::hello)
            .service(handlers::test::echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
