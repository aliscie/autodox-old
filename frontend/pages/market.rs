use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}
#[function_component(Market)]
pub fn market(props: &Props) -> Html {
    html! { <h1>{"Market"}</h1> }
}
