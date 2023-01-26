use crate::router::Route;
use serde::{Deserialize, Serialize};
use shared::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent, Node};
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_toggle};
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub items: Vec<Html>,
    pub position: UseStateHandle<Option<MouseEvent>>,
    pub click_on: Option<bool>,
}

#[function_component(PopOverMenu)]
pub fn menu(props: &MenuProps) -> Html {
    let _popover = NodeRef::default();
    let _clicked_element: Option<Element> = if let Some(p) = &*props.position {
        Some(p.target_unchecked_into::<Element>())
    } else {
        None
    };

    let display = if let Some(p) = &*props.position {
        let width = window()
            .unwrap_throw()
            .inner_width()
            .unwrap_throw()
            .unchecked_into_f64();
        let popover_width = 155;
        let right = &p.page_x() + popover_width;
        let mut x = p.page_x();
        if right > width as i32 {
            x -= popover_width;
        }
        format!(
            "display : block; top:{}px; left:{}px;",
            &p.page_y(),
            &x
        )
    } else {
        String::from("display: none;")
    };

    let position = props.position.clone();
    let clicked_element = _clicked_element.clone();
    let popover = _popover.clone();
    let click_on = props.click_on.clone();
    use_event_with_window("click", move |_e: MouseEvent| {
        let element = popover.cast::<Node>();
        let target = &_e.target_unchecked_into::<Element>();
        let target_node = &_e.target_unchecked_into::<Node>();
        let mut is_click_on: bool = element.clone().unwrap().contains(Some(target_node));
        if click_on.is_none() {
            is_click_on = false;
        }
        if let Some(element) = &clicked_element.clone() {
            if element != target && !is_click_on {
                &position.set(None);
            }
        }
    });

    return html! {
        <div
        ref={_popover}
        style={format!("{}", display)}
        class={"dropdown-content"}
        >
            {props.items.clone()}
        </div>
    };
}
