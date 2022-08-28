use yew::{function_component, html, Properties};

use crate::api::Message;

#[derive(Properties, PartialEq, Eq)]
pub struct ChatMessageProps {
    pub message: Message,
    pub my_id: i64,
}

#[function_component(ChatMessage)]
pub fn _chat_message(ChatMessageProps { message, my_id }: &ChatMessageProps) -> Html {
    let label = format!(
        "User#{} at {}",
        message.get_actor(),
        message.get_created_at()
    );

    let is_my_message = *my_id == message.get_actor() as i64;
    let class = if is_my_message {
        "ui right red ribbon label"
    } else {
        "ui left red ribbon label"
    };
    html! {
        <div class="ui segment">
        <a class={class}>{label}</a>
        <span>{message.get_text()}</span>
        </div>
    }
}
