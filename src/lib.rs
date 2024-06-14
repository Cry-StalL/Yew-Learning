use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component]
pub fn App() -> Html {
    html! {
        <div class={css!("color: red;")}>
            {"Hello World!"}
        </div>
    }
}