use actix_web::{web::Data, App, HttpServer};

use actix_web::middleware::Logger;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger::Env;

mod handlers;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // r2d2 pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Cannot create r2d2 pool");

    let host = env::var("HOST").expect("HOST variable not set!");
    let port = env::var("PORT")
        .expect("PORT variable not set!")
        .parse::<u16>()
        .expect("Cannot parse PORT as integer!");

    println!("Chat server run at http://{}:{}", host, port.to_string());

    HttpServer::new(move || {
        App::new()
            // add r2d2 pool
            .app_data(Data::new(pool.clone()))
            // logging
            .wrap(Logger::new("%a %{User-Agent}i %r %s, %T secs"))
            // API
            // get all messages
            .service(handlers::messages::get_messages)
            // post message
            .service(handlers::messages::post_message)
            // read new messages
            // health
            .service(handlers::health::health)
    })
    .bind((host, port))
    .expect("host/addr already in use!")
    .run()
    .await
}
