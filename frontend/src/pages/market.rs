use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn Market(props: &Props) -> Html {
    html! { <h1>{"Market"}</h1> }
}
