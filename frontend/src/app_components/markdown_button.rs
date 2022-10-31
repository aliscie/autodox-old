use serde::{Deserialize, Serialize};
use shared::invoke;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
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
        _position.set(Some(_e));
    });

    let items: Vec<Html> = vec![
        html! {<>
        <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike1"/>
          <label for="vehicle1">{"Show marks"}</label>
          </>},
        html! {<>
        <input type="checkbox" id="vehicle2" name="vehicle2" value="Bike2"/>
          <label for="vehicle2">{"Show html"}</label>
          </>},
        html! {<>
        <input type="checkbox" id="vehicle3" name="vehicle3" value="Bike3"/>
          <label for="vehicle3">{"Show render"}</label>
          </>},
    ];

    html! {<>
        <Menu click_on={Some(true)} event={position.clone()}{items}/>
        <li {onmouseup} class="btn right_clickable"> <i class="fa-brands fa-markdown"></i></li>
    </>}
}
