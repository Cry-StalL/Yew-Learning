use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! { "Hello World!" }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
