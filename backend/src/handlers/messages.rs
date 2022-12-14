use crate::{
    entities::message::{Message, NewMessage},
    types::{DbPool, DieselError, PoolConnection},
};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse, Responder, Scope};
use chrono::NaiveDateTime;
use diesel::{QueryDsl, RunQueryDsl};
use lib::dto::{PostMessageDtoResponse, UserExistsDto};

use super::scope;

pub fn api() -> Scope {
    scope(Some("messages"))
        .service(post_message)
        .service(get_messages)
        .service(get_new_messages)
        .service(is_user_exists)
}

async fn get_all_messages(connection: PoolConnection) -> Result<Vec<Message>, DieselError> {
    use crate::schema::messages::dsl::*;
    messages.load::<Message>(&connection)
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

#[get("")]
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

#[get("new/{timestamp}")]
pub async fn get_new_messages(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let connection = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Cannot get connection from pool")
        }
    };
    let timestamp = match req.match_info().get("timestamp").unwrap().parse::<i64>() {
        Ok(ts) => ts,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::BAD_REQUEST)
                .body("InvalidTimestamp")
        }
    };
    if timestamp <= 0 {
        return HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .body("InvalidTimestamp");
    }

    let results = get_newest_messages(connection, timestamp).await;
    match results {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Cannot fetch messages"),
    }
}

async fn get_newest_messages(
    connection: PoolConnection,
    timestamp: i64,
) -> Result<Vec<Message>, DieselError> {
    use crate::diesel::ExpressionMethods;
    use crate::schema::messages::dsl::*;
    messages
        .filter(created_at.gt(NaiveDateTime::from_timestamp(timestamp, 0)))
        .load::<Message>(&connection)
}

#[get("/user/{id}")]
pub async fn is_user_exists(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let connection = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Cannot get connection from pool")
        }
    };
    let user_id = match req.match_info().get("id").unwrap().parse::<i32>() {
        Ok(ts) => ts,
        Err(_) => {
            return HttpResponse::Ok()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid userId")
        }
    };
    if user_id <= 0 {
        return HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid userId");
    }

    let is_exists = _is_user_exists(connection, user_id).await;
    HttpResponse::Ok().json(UserExistsDto::new(is_exists))
}

async fn _is_user_exists(connection: PoolConnection, user_id: i32) -> bool {
    use crate::diesel::ExpressionMethods;
    use crate::schema::messages::dsl::*;
    let record = messages
        .filter(actor.eq(user_id))
        .first::<Message>(&connection);
    record.is_ok()
}
