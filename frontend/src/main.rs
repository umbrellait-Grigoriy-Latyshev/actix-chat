use gloo_net::{http::Request, Error};
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
                <p>{format!("{}", message.text)}</p>
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
            <h1>{ "Simple Rust chat" }</h1>
            <MessageList messages={(*messages).clone()} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
