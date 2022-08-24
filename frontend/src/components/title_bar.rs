use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Element, MouseEvent, window, Document};

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: String,
    pub children: Children,

}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    html! {
    <div class="custom_title_bar">
       <div class="buttons">
            {props.children.clone()}
        </div>
        <span style="margin-left:5%">{props.title.clone()}</span>
    </div>
    }
}
