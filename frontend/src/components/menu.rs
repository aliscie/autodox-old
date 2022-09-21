use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::prelude::*;
use web_sys::console::log_1;
use crate::router::Route;

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
        display = format!(
            "position: absolute; display: block; top:{}px; right:{}px;",
            _e.offset_y(), _e.offset_x()
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
