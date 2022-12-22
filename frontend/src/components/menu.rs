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
    pub position: UseStateHandle<Option<MouseEvent>>,
}

#[function_component(PopOverMenu)]
pub fn menu(props: &MenuProps) -> Html {
    let popover = NodeRef::default();
    let clicked_element: Option<Element> = if let Some(p) = &*props.position {
        Some(p.target_unchecked_into::<Element>())
    } else {
        None
    };


    let display = if let Some(p) = &*props.position {
        let width = window().unwrap_throw().inner_width().unwrap_throw().unchecked_into_f64();
        let popover_width = 155;
        let right = &p.page_x() + popover_width;
        let mut x = p.page_x();
        if right > width as i32 {
            x -= popover_width;
        }
        format!(
            "display : block; top:{}px; left:{}px; z-index: 10000;",
            &p.page_y(), &x
        )
    } else {
        String::from("display: none;")
    };
    let position = props.position.clone();
    let _clicked_element = clicked_element.clone();
    use_event_with_window("click", move |_e: MouseEvent| {
        if let Some(element) = &_clicked_element.clone() {
            if element != &_e.target_unchecked_into::<Element>() {
                &position.set(None);
            }
        }
    });
    return html! {
    <div
        ref={popover}
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
