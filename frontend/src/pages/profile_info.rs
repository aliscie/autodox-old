use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component(ProfileInfo)]
pub fn profile_info(props: &Props) -> Html {
    html! { <h1>{"ProfileInfo"}</h1> }
}
