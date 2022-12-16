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

use crate::components::ContextMenu;
use crate::router::Route;
use crate::utils::DeviceInfo;
use crate::*;

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component(Download)]
pub fn download(props: &DownloadProps) -> Html {
    let position: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);
    let onmouseup: Callback<MouseEvent> = {
        let position = position.clone();
        Callback::from(move |_e: MouseEvent| {
            position.set(None);
        })
    };
    let onclick = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            let y = e.page_y();
            let x = e.page_x();
            position.set(Some((y, x)));
        })
    };

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-brands fa-apple"></i>{"Mac"}</a>},
        html! {<a><i class="fa-brands fa-windows"></i>{"Window"}</a>},
        html! {<a><i class="fa-brands fa-ubuntu"></i>{"Linux"}</a>},
    ];

    return html! {
    <>
        <ContextMenu items = {items} position = {position.clone()}/>
        <span  {onclick} {onmouseup} class="btn" ><i class="fa-solid fa-download"></i>{"Download"}</span>
    </>
    };
}
