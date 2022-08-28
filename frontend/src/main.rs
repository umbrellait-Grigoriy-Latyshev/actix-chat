use crate::components::input_btn::InputBtn;
use crate::components::message_list::MessageList;
use chrono::Utc;
use yew::prelude::*;

mod api;
mod components;

#[function_component(App)]
fn app() -> Html {
    let me_id = Utc::now().timestamp();
    html! {
        <>
            <components::header::Header/>
            <MessageList my_id={me_id} />
            <InputBtn my_id={me_id}/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
