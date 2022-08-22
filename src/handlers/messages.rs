use actix::entities::message::Message;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    handlers::dto::{PostMessageDto, PostMessageDtoResponse},
    DbPool,
};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

#[post("/message")]
pub async fn post_message(info: web::Json<PostMessageDto>) -> impl Responder {
    println!("Tried to post '{}'", info.message);
    HttpResponse::Ok().json(PostMessageDtoResponse::new())
}

async fn get_all_messages(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<Message> {
    use actix::schema::messages::dsl::*;
    let result = messages
        .limit(5)
        .load::<Message>(&connection)
        .expect("Error loading messages");
    result
}

#[get("/messages")]
pub async fn get_messages(pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let results = get_all_messages(connection).await;
    HttpResponse::Ok().json(results)
}
