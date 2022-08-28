use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq, Eq)]
pub struct InputBtnProps {
    pub my_id: i64,
}

#[function_component(InputBtn)]
pub fn _sendbtn(props: &InputBtnProps) -> Html {
    let actor_id = props.my_id;
    let onclick = {
        Callback::from(move |_| {
            spawn_local(async move {
                crate::api::post_messages(actor_id, "text 12".to_string())
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <div class="ui action input">
                <input type={"text"} placeholder={"Your awesome message..."}/>
                <button {onclick} class="ui button">{"Send"}</button>

        </div>
    }
}
