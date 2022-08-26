use actix_web::{web, Scope};

pub mod health;
pub mod messages;

pub fn api() -> Scope {
    web::scope("/api")
        .service(messages::api())
        .service(health::api())
}

pub fn scope(path: Option<&str>) -> Scope {
    web::scope(path.unwrap_or(""))
}
