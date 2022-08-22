use crate::{
    entities::message::{Message, NewMessage},
    types::{DbPool, DieselError, PoolConnection},
};
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};

use crate::handlers::dto::PostMessageDtoResponse;
use diesel::prelude::*;

async fn get_all_messages(connection: PoolConnection) -> Result<Vec<Message>, DieselError> {
    use crate::schema::messages::dsl::*;
    messages.limit(5).load::<Message>(&connection)
}

async fn create_message(
    connection: PoolConnection,
    actor: i32,
    text: String,
) -> Result<usize, DieselError> {
    use crate::schema::messages;
    let new_message = NewMessage { actor, text };
    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&connection)
}

#[post("/message")]
pub async fn post_message(pool: web::Data<DbPool>, info: web::Json<NewMessage>) -> impl Responder {
    let connection = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Cannot get connection from pool")
        }
    };
    let actor = info.actor;
    let text = info.text.clone();
    let insert = create_message(connection, actor, text).await;
    match insert {
        Ok(_) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .json(PostMessageDtoResponse::new(true)),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(PostMessageDtoResponse::new(false)),
    }
}

#[get("/messages")]
pub async fn get_messages(pool: web::Data<DbPool>) -> impl Responder {
    let connection = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Cannot get connection from pool")
        }
    };
    let results = get_all_messages(connection).await;
    match results {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Cannot fetch messages"),
    }
}
