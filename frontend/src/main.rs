use serde::Deserialize;
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
    let messages = vec![
        Message {
            actor: 1,
            text: "test 1".to_string(),
        },
        Message {
            actor: 2,
            text: "text 2".to_string(),
        },
    ];
    html! {
        <>
            <h1>{ "Simple Rust chat" }</h1>
            <MessageList messages={messages} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
