use gloo_net::{http::Request, Error};
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::chat_message::*;
mod components;

#[derive(Clone, PartialEq, Deserialize)]
struct Message {
    actor: i32,
    text: String,
}

#[derive(Properties, PartialEq)]
struct MessageListProps {
    messages: Vec<Message>,
}

async fn api_messages() -> Result<Vec<Message>, Error> {
    let url = "/api/messages";
    let resp = Request::get(url).send().await.unwrap();
    let data = resp.json().await;
    data
}

#[function_component(MessageList)]
fn message_list(MessageListProps { messages }: &MessageListProps) -> Html {
    messages
        .iter()
        .map(|message| {
            html! {
                <ChatMessage author={message.actor} text={message.text.clone()}></ChatMessage>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(|| vec![]);
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                let messages = messages.clone();

                spawn_local(async move {
                    let fetched = api_messages().await.unwrap();
                    messages.set(fetched);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
        <components::header::Header/>
            <MessageList messages={(*messages).clone()} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
