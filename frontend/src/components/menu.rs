use serde::{Deserialize, Serialize};
use shared::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent, Node};
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_toggle};
use yewdux::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub items: Vec<Html>,
    pub position: UseStateHandle<Option<(i32, i32)>>,
}

#[function_component(ContextMenu)]
pub fn menu(props: &MenuProps) -> Html {
    let display = if let Some((y, x)) = *props.position {
        // z-index to make it float over everything else!
        format!(
            "display : block; top:{}px; left:{}px; z-index: 10000;",
            &y, &x
        )
    } else {
        String::from("display: none;")
    };
    let position = props.position.clone();
    use_event_with_window("click", move |_e: MouseEvent| {
        position.set(None);
    });
    return html! {
    <div
        style={format!("{}", display)}
        class={"dropdown-content"}
    >
        {
            props.items.clone().into_iter().map(|item| {
                item
            }).collect::<Html>()
        }
    </div>
     };
}
