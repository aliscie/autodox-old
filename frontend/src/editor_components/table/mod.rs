use wasm_bindgen::{UnwrapThrowExt};
use yew::{function_component, html};
use yew::prelude::*;
use web_sys::{MouseEvent, window};

use crate::components::PopOverMenu;

#[wasm_bindgen(module = "/src/editor_components/table/table_insertions.js")]
extern "C" {
    #[wasm_bindgen(js_name = insCol)]
    pub fn insCol(id: String);

    #[wasm_bindgen(js_name = insRow)]
    pub fn insRow(id: String);

    #[wasm_bindgen(js_name = deleteCol)]
    pub fn deleteCol(id: String);

    #[wasm_bindgen(js_name = deleteRow)]
    pub fn deleteRow(id: String);
}


#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
}


#[function_component]
pub fn Table(props: &Props) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let onclick = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
        })
    };
    let items: Vec<Html> = vec![
        html! {<a><i class="fa-brands fa-apple"></i>{"add column ->"}</a>},
        html! {<a><i class="fa-brands fa-apple"></i>{"add column <-"}</a>},
        html! {<a><i class="fa-brands fa-apple"></i>{"add row ^"}</a>},
        html! {<a><i class="fa-brands fa-apple"></i>{"add row bellow"}</a>},
    ];

    html! {<span  {onclick}>
        <PopOverMenu {items} position = {position.clone()}/>
        <table>{"hello"}</table>
        </span>}
}