use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

// use shared::{invoke, log};

use crate::components::PopOverMenu;
use crate::router::Route;
use crate::specific_components::file_data::UseChangeHandle;
use crate::utils::DeviceInfo;
use crate::*;

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component]
pub fn SaveButton(props: &DownloadProps) -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    // let changes = use_store_value::<UseChangeHandle>();
    // log!(&changes.changes);
    // let is_empty = changes.changes.is_empty();
    // log!(is_empty);
    let onclick = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {
            dispatch.reduce_mut(|state| state.is_saved = true);
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
        </span>}
    }
    if device.is_web {
        return html! {
                <span {onclick}>
            {save_mark}
                </span>
        };
    }

    return html! {<></>};
}
