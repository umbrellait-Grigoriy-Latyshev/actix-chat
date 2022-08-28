use yew::{function_component, html, Properties};

use super::chat_message::ChatMessage;
use crate::api::Message;

#[derive(Properties, PartialEq, Eq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
}

#[function_component(MessageList)]
pub fn message_list(MessageListProps { messages }: &MessageListProps) -> Html {
    messages
        .iter()
        .map(|message| {
            html! {
                <ChatMessage author={message.get_actor()} text={message.get_text()}></ChatMessage>
            }
        })
        .collect()
}
