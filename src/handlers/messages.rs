use crate::entities::message::Message;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};

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
) -> Result<Vec<Message>, diesel::result::Error> {
    use crate::schema::messages::dsl::*;
    let result = messages.limit(5).load::<Message>(&connection);
    result
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
    let response = match results {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Cannot fetch messages"),
    };
    response
}
