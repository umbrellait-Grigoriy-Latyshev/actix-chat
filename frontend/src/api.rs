use chrono::NaiveDateTime;
use gloo_net::{
    http::{Request, Response},
    Error,
};
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Properties, Clone, PartialEq, Eq, Deserialize)]
pub struct Message {
    actor: i64,
    text: String,
    created_at: i64,
}

impl Message {
    pub fn get_actor(&self) -> i64 {
        self.actor
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_created_at(&self) -> String {
        NaiveDateTime::from_timestamp(self.created_at, 0)
            .format("%a %b %e %T %Y")
            .to_string()
    }
}

pub async fn api_messages() -> Result<Vec<Message>, Error> {
    let url = "/api/messages";
    let resp = Request::get(url).send().await.unwrap();
    resp.json().await
}

#[derive(Serialize)]
pub struct NewMessage {
    pub actor: i64,
    pub text: String,
}

pub async fn post_messages(actor: i64, text: String) -> Result<Response, Error> {
    let url = "/api/messages/message";
    let body = NewMessage {
        actor,
        text: text.clone(),
    };
    Request::post(url).json(&body).unwrap().send().await
}
