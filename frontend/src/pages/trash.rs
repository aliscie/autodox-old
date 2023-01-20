use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(Trash)]
pub fn trash(props: &Props) -> Html {
    html! {<h1>{"Trash"}</h1>}
}
