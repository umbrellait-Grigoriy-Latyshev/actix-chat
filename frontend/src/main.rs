use crate::components::input_btn::InputBtn;
use crate::components::message_list::MessageList;
use yew::prelude::*;

mod api;
mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <components::header::Header/>
            <MessageList  />
            <InputBtn />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
