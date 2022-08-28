use gloo_net::{
    http::{Request, Response},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Message {
    actor: i32,
    text: String,
}

impl Message {
    pub fn get_actor(&self) -> i32 {
        self.actor
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

pub async fn api_messages() -> Result<Vec<Message>, Error> {
    let url = "/api/messages";
    let resp = Request::get(url).send().await.unwrap();
    resp.json().await
}

#[derive(Serialize)]
pub struct NewMessage {
    pub actor: i32,
    pub text: String,
}

pub async fn post_messages(actor: i32, text: String) -> Result<Response, Error> {
    let url = "/api/messages/message";
    let body = NewMessage {
        actor,
        text: text.clone(),
    };
    Request::post(url).json(&body).unwrap().send().await
}
