use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}
#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    html! { <h1>{"Settings"}</h1> }
}
