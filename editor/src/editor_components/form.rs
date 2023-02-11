use wasm_bindgen::{UnwrapThrowExt};
use yew::{function_component, html};
use yew::prelude::*;
use web_sys::{MouseEvent, window};


#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
}


#[function_component]
pub fn FromComponent(props: &Props) -> Html {
    html! {<table>{"hello from"}</table>}
}