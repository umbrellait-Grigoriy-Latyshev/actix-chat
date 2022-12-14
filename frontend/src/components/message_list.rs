use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};

use super::chat_message::ChatMessage;
use crate::api::Message;

#[derive(Properties, PartialEq, Eq)]
pub struct MessageListProps {
    pub my_id: i64,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    let messages = use_state(Vec::<Message>::new);

    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                let messages = messages;

                spawn_local(async move {
                    let fetched = crate::api::api_messages().await.unwrap();
                    messages.set(fetched);
                });
                || ()
            },
            (),
        );
    }

    messages
        .iter()
        .map(|message| {
            html! {
                <ChatMessage my_id={props.my_id} message={message.clone()}></ChatMessage>
            }
        })
        .collect()
}
