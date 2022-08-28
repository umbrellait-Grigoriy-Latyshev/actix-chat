use gloo_net::{http::Request, Error};
use serde::Deserialize;

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
