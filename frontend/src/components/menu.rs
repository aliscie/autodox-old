use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, Node, window};
use yew::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub items: Vec<Html>,
    pub event: UseStateHandle<Option<MouseEvent>>,
    pub click_on: Option<bool>,
    pub clicked_element: Option<UseStateHandle<Option<Element>>>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let node_ref = NodeRef::default();
    let doc = window().unwrap_throw().document().unwrap();

    let event = (*props.event).clone();

    let _event = props.event.clone();
    let _doc = doc.clone();
    let _click_on = props.click_on.clone();
    let _node_ref = node_ref.clone();
    let _clicked_element = props.clicked_element.clone();
    let click_away_handler = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
        let curr: Node = _e.target_unchecked_into();
        let mut hide = true;
        let mut drop_down = _node_ref.clone().cast::<Element>();

        if &drop_down != &None {
            let drop_down = drop_down.unwrap();
            // TODO use &curr instead of some(&curr)
            let is_dropdown_element = drop_down.contains(Some(&curr));
            if is_dropdown_element {
                hide = !(&_click_on != &None && &_click_on.unwrap() == &true);
                let curr: Element = _e.target_unchecked_into();
                // if _clicked_element.clone() != None { &_clicked_element.unwrap().set(Some(curr)) };
            }
            if *_event != None && hide {
                &_event.set(None);
            }
        }
    }) as Box<dyn FnMut(_)>);

    let _ = &doc
        .query_selector("#app")
        .unwrap()
        .unwrap()
        .add_event_listener_with_callback(
            "mousedown",
            &click_away_handler.as_ref().unchecked_ref(),
        );

    click_away_handler.forget();
    let mut display: String = "display: none".to_string();
    let mut p = "".to_string();

    if &event != &None {
        let _e = &event.clone().unwrap();

        let mut y = _e.offset_y();
        let mut x = _e.page_x();
        let menu_width = 130;
        let window_width =
            JsValue::as_f64(&window().unwrap_throw().inner_width().unwrap()).unwrap() as i32;
        if &x + &menu_width > window_width {
            x = x - &menu_width;
        };

        display = format!("display: block; top:{}px; left:{}px", &y, &x).to_string();
    };

    return html! {
    <div
        ref={node_ref}
        style={format!("{};", display)}
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
