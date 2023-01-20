use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(ProfileInfo)]
pub fn profile_info(props: &Props) -> Html {
    html! {<h1>{"ProfileInfo"}</h1>}
}
