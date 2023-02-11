use crate::components::PopOverMenu;
use crate::router::Route;
use crate::*;
use serde::{Deserialize, Serialize};
use shared::invoke;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Markdown)]
pub fn markdown() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let _position = position.clone();
    let on_click: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let items: Vec<Html> = vec![
        html! {<a>
            <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike1"/>
            <label for="vehicle1">{"Show marks"}</label>
        </a>},
        html! {<a>
            <input type="checkbox" id="vehicle2" name="vehicle2" value="Bike2"/>
            <label for="vehicle2">{"Show html"}</label>
        </a>},
        html! {<a>
            <input type="checkbox" id="vehicle3" name="vehicle3" value="Bike3"/>
            <label for="vehicle3">{"Show render"}</label>
        </a>},
    ];

    html! {<>
        <PopOverMenu click_on={Some(true)} items = {items} position = {position.clone()}/>
        <li onclick={on_click} class="btn right_clickable"><i class="fa-brands fa-markdown"></i></li>
    </>}
}
