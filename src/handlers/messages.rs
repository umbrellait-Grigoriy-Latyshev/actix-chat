use actix::entities::message::Message;
use actix::establish_connection;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::handlers::dto::{PostMessageDto, PostMessageDtoResponse};
use diesel::prelude::*;

#[post("/message")]
pub async fn post_message(info: web::Json<PostMessageDto>) -> impl Responder {
    println!("Tried to post '{}'", info.message);
    HttpResponse::Ok().json(PostMessageDtoResponse::new())
}

#[get("/messages")]
pub async fn get_messages() -> impl Responder {
    use actix::schema::messages::dsl::*;

    let connection = establish_connection();
    let results = messages
        .limit(5)
        .load::<Message>(&connection)
        .expect("Error loading messages");

    println!("messages length: {}", results.len());
    HttpResponse::Ok().json(results)
}
