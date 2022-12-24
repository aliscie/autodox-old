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
use crate::components::PopOverMenu;
use crate::router::Route;
use crate::utils::DeviceInfo;

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component(Download)]
pub fn download(props: &DownloadProps) -> Html {
    let (device, _) = use_store::<DeviceInfo>();

    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let onclick = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
        })
    };

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-brands fa-apple"></i>{"Mac"}</a>},
        html! {<a href={"blob:https://mega.nz/9b38ed11-8f43-404b-87d5-eb7b2ac37692"} target="_blank"><i class="fa-brands fa-windows"></i>{"Windows"}</a>},
        html! {<a><i class="fa-brands fa-ubuntu"></i>{"Linux"}</a>},
    ];
    let mut res = html! {};

    if device.is_web {
        res = html! {
        <>
            <PopOverMenu items = {items} position = {position.clone()}/>
            <span  {onclick} class="btn" ><i class="fa-solid fa-download"></i>{"Download"}</span>
        </>
    };
    }

    return res;
}
