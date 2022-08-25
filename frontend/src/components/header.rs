use yew::{function_component, html};

#[function_component(Header)]
pub fn _header() -> Html {
    html! {
        <h1 class="ui header">{"Simple Rust app!"}</h1>
    }
}
