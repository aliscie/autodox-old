use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use shared::invoke;

use crate::*;
use crate::components::ContextMenu;
use crate::router::Route;
use crate::utils::DeviceInfo;

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component(Download)]
pub fn download(props: &DownloadProps) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let onclick = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
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
        <span  {onclick} class="btn" ><i class="fa-solid fa-download"></i>{"Download"}</span>
    </>
    };
}
