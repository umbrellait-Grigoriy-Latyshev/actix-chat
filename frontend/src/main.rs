use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::message_list::MessageList;

mod api;
mod components;

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(Vec::new);
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                let messages = messages;

                spawn_local(async move {
                    let fetched = api::api_messages().await.unwrap();
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
