use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub display: UseStateHandle<bool>,
    pub items:  Vec<Html>,
}


#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {

    // TODO Be carefully previously the app freeze when I uncomment this?
    //  it freeze after clicking 3 time son a file.


    let x = props.display.clone();
    let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if (*x).clone() == true {
            &x.set(false);
        }
    }) as Box<dyn FnMut(_)>);

    let doc = window().unwrap_throw().document().unwrap();
    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("click", &click_away_handler.as_ref().unchecked_ref());
    click_away_handler.forget();
    let _display =  if *props.display {"display: block"} else {"display: none"};

    html! {
    <div
        style={format!("z-index: 99; {}", _display)}
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
