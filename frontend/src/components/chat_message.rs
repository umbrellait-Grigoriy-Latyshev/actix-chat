use yew::{function_component, html, Properties};

use crate::api::Message;

#[derive(Properties, PartialEq, Eq)]
pub struct ChatMessageProps {
    pub message: Message,
}

#[function_component(ChatMessage)]
pub fn _chat_message(ChatMessageProps { message }: &ChatMessageProps) -> Html {
    let label = format!(
        "User#{} at {}",
        message.get_actor(),
        message.get_created_at()
    );
    html! {
        <div class="ui segment">
        <a class="ui red ribbon label">{label}</a>
        <span>{message.get_text()}</span>
        </div>
    }
}
