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
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component(Download)]
pub fn download(props: &DownloadProps) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let items: Vec<Html> = vec![
        html! {<><i class="fa-brands fa-apple"></i>{"Mac"}</>},
        html! {<><i class="fa-brands fa-windows"></i>{"Window"}</>},
        html! {<><i class="fa-brands fa-ubuntu"></i>{"Linux"}</>},
    ];

    if *IS_WEB {
        html! {<>
            <Menu event={position.clone()}{items}/>
            <span  {onmouseup} class="btn" ><i class="fa-solid fa-download"></i>{"Download"}</span>
        </>}
    } else {
        html! {""}
    }
}
