use serde::{Deserialize, Serialize};
#[cfg(not(feature = "web"))]
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

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}


#[function_component(Download)]
pub fn download(props: &DownloadProps) -> Html {
    let position: UseStateHandle<String> = use_state(|| "".to_string());
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(
            format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
        );
    });

    let items: Vec<Html> = vec![
        html! {<><i class="fa-brands fa-apple"></i>{"Mac"}</>},
        html! {<><i class="fa-brands fa-windows"></i>{"Window"}</>},
        html! {<><i class="fa-brands fa-ubuntu"></i>{"Linux"}</>},
    ];

    let mut is_web = true;
    #[cfg(not(feature = "web"))] {
        is_web = false;
    };
    {
        if is_web {
        html!{<>
            <Menu position={position.clone()}{items}/>
            <span  {onmouseup} class="btn" ><i class="fa-solid fa-download"></i>{"Download"}</span>
        </>}
        } else {html!{""}}
    }

}
