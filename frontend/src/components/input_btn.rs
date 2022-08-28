use yew::{function_component, html};

#[function_component(InputBtn)]
pub fn _sendbtn() -> Html {
    html! {
        <div class="ui action input">
                <input type={"text"} placeholder={"Your awesome message..."}/>
                <button class="ui button">{"Send"}</button>
        </div>
    }
}
