use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::prelude::*;
use web_sys::console::log_1;
use crate::router::Route;
use wasm_bindgen::JsValue;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub items: Vec<Html>,
    pub event: UseStateHandle<Option<MouseEvent>>,

}


#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let doc = window().unwrap_throw().document().unwrap();


    let event = (*props.event).clone();

    let _event = props.event.clone();
    let click_away_handler = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
        if *_event != None {
            &_event.set(None);
        }
    }) as Box<dyn FnMut(_)>);


    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("mousedown", &click_away_handler.as_ref().unchecked_ref());

    click_away_handler.forget();
    let mut display: String = "display: none".to_string();
    let mut p = "".to_string();

    if &event != &None {
        let _e = &event.clone().unwrap();

        let mut y = _e.offset_y();
        let mut x = _e.page_x();
        let menu_width = 130;
        let window_width = JsValue::as_f64(&window().unwrap_throw().inner_width().unwrap()).unwrap() as i32;
        if &x + &menu_width > window_width {
            x = x - &menu_width;
        };


        display = format!(
            "position: absolute; display: block; top:{}px; left:{}px; animation-duration: 0.2s; animation-fill-mode: both; animation-name: btnEntrance;",
            &y, &x
        ).to_string();
    }


    html! {
    <div
        style={format!("event: absolute; {};", display)}
        class={"dropdown-content"}
    >
        {
            props.items.clone().into_iter().map(|item| {
                html!{
                <a>{item}</a>
                }
            }).collect::<Html>()
        }

    </div>
     }
}
