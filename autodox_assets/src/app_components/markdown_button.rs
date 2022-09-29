use serde::{Deserialize, Serialize};
use shared::invoke;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use web_sys::console::log_1;
use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::Menu;
use crate::router::Route;
use crate::*;

#[derive(PartialEq, Properties)]
pub struct MarkdownProps {
    // pub id: u64,
}


#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(
            Some(_e)
        );
    });

    let items: Vec<Html> = vec![
        html! {<>
        <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike"/>
          <label for="vehicle1">{"Show marks"}</label>
          </>},
        html! {<>
        <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike"/>
          <label for="vehicle1">{"Show html"}</label>
          </>},
        html! {<>
        <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike"/>
          <label for="vehicle1">{"Show render"}</label>
          </>},
    ];

    html! {<>
            <Menu click_on={Some(true)} event={position.clone()}{items}/>
            <li {onmouseup} class="btn right_clickable"> <i class="fa-brands fa-markdown"></i></li>
        </>}
}
