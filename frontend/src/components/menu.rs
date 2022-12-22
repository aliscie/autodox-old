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
        let popover_width: f64 = 100 as f64;
        // let right: f64 = popover.clone().cast::<Element>().unwrap().get_bounding_client_rect().x();
        crate::shared::log!(format!("right {:#?}, {:#?}",&p.page_x()+popover_width, &width).to_string());
        format!(
            "display : block; top:{}px; left:{}px; z-index: 10000;",
            &p.page_y(), &p.page_x()
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
