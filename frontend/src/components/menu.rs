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
    pub position: UseStateHandle<String>,

}


#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let doc = window().unwrap_throw().document().unwrap();


    let _position = props.position.clone();
    let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if (*_position).clone().len() > 0 {
            &_position.set("".to_string());
        }
    }) as Box<dyn FnMut(_)>);


    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("mousedown", &click_away_handler.as_ref().unchecked_ref());
    click_away_handler.forget();
    let mut _display = "display: none";
    let mut p = "".to_string();

    if (*props.position).clone().len() > 0 {
        p = (*props.position).clone().to_string();
        _display = "display: block"
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
