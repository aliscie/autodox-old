mod profile_info;
mod market;

use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component]
pub fn Page(props: &Props) -> Html {
    match props.name.clone().as_ref() {
        "profile" => html! { <profile_info::ProfileInfo/> },
        "market" => html! { <market::Market/> },
        _ => html! { <span>{"404"}</span> },
    }
}
