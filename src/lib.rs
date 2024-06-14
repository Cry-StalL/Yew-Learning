use yew::prelude::*;
use stylist::yew::use_style;

#[function_component]
pub fn App() -> Html {
    let style = use_style!("color: red; font-size: 20px;");
    html! {
        <div class={style}>
            {"Hello World!"}
        </div>
    }
}