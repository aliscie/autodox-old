use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
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

#[function_component]
pub fn SaveButton(props: &DownloadProps) -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();

    let onclick = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {
            // let res = backend::save_elements_ic(device.updates).await;
            dispatch.reduce_mut(|state| state.is_saved = true);
            ;
        })
    };

    let mut save_mark = html! {
        <span class=" btn" >
            <i style="color: tomato" class="fa-solid fa-x"/>
            {"Save"}
        </span>
        };
    if device.is_saved {
        save_mark = html! {<span class=" btn"   >
            <i style="color: lightgreen" class="fa-solid fa-check"/>
            {"Saved"}
        </span>} }
    if device.is_web {
        return html! {
            <span {onclick}>
        {save_mark}
            </span>
    };
    }

    return html! {<></>};
    ;
}
