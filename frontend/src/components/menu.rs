use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::prelude::*;
use web_sys::console::log_1;
use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub display: UseStateHandle<bool>,
    pub items: Vec<Html>,
    pub position: Option<UseStateHandle<String>>,

}


#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let doc = window().unwrap_throw().document().unwrap();



    let _display = props.display.clone();
    let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if (*_display).clone() == true {
            &_display.set(false);
        }
    }) as Box<dyn FnMut(_)>);


    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("mousedown", &click_away_handler.as_ref().unchecked_ref());
    click_away_handler.forget();
    let _display = if *props.display { "display: block" } else { "display: none" };
    let mut p = "".to_string();

    // TODO 🔴
    //     get mouse position from yewdux
    if props.position.clone() != None {
        p = props.position.clone().unwrap().to_string();
    };

    html! {
    <div
        style={format!("position: absolute; {}; {}", _display, p)}
        class={"dropdown-content"}
    >
        {
            props.items.clone().into_iter().map(|item| {
                html!{
                <a href="#">{item}</a>
                }
            }).collect::<Html>()
        }

    </div>
     }
}
