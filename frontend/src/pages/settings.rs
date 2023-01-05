use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component]
pub fn Settings(props: &Props) -> Html {
    html! { <h1>{"Settings"}</h1> }
}
