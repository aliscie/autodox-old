use serde::{Deserialize, Serialize};
use shared::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent, Node};
use yew::prelude::*;
use yew_hooks::use_toggle;

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
        "display : none".to_string()
    };
    log!(&display);
    let position = props.position.clone();
    let onmouseleave = Callback::from(move |e: MouseEvent| {
        let element = e.target_unchecked_into::<Element>();
        element.set_attribute("style", "display: none");
        position.set(None);
    });
    return html! {
    <div
        {onmouseleave}
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
