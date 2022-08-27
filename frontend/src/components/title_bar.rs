use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Document, Element, MouseEvent};

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: String,
    pub children: Children,
}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    return html! {
        <div class="custom_title_bar">
           <div class="buttons">
                {props.children.clone()}
            </div>
            <span style="margin-left:5%">{props.title.clone()}</span>
        </div>
    };
}
