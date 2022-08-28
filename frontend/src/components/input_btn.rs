use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback};

#[function_component(InputBtn)]
pub fn _sendbtn() -> Html {
    let onclick = {
        Callback::from(move |_| {
            spawn_local(async move {
                crate::api::post_messages(12, "text 12".to_string())
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
