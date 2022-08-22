use actix_web::{post, web, HttpResponse, Responder};

use crate::handlers::dto::{PostMessageDto, PostMessageDtoResponse};

#[post("/post")]
pub async fn post_message(info: web::Json<PostMessageDto>) -> impl Responder {
    println!("Tried to post '{}'", info.message);
    HttpResponse::Ok().json(PostMessageDtoResponse::new())
}
