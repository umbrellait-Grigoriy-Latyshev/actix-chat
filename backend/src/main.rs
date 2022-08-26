#[macro_use]
extern crate diesel;

use actix_web::{web::Data, App, HttpServer};

use actix_web::middleware::Logger;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger::Env;

mod entities;
mod handlers;
mod schema;
mod types;
use std::env;

use crate::handlers::api;

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

    println!("Chat server run at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::new("%a %{User-Agent}i %r %s, %T secs"))
            .service(api())
    })
    .bind((host, port))
    .expect("host/addr already in use!")
    .run()
    .await
}
