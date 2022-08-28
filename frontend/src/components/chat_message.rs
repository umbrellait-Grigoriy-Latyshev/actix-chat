use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq, Eq)]
pub struct ChatMessageProps {
    pub author: i32,
    pub text: String,
}

#[function_component(ChatMessage)]
pub fn _chat_message(ChatMessageProps { author, text }: &ChatMessageProps) -> Html {
    html! {
        <div class="ui segment">
        <a class="ui red ribbon label">{"User#"}{author}</a>
        <span>{text}</span>
        </div>
    }
}
