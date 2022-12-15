use serde::{Deserialize, Serialize};
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

#[derive(PartialEq, Properties)]
pub struct PageOptionsProps {
    // pub id: u64,
}

#[function_component(PageOptions)]
pub fn page_options(props: &PageOptionsProps) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-comment"></i>{"Comments"}</a>},
        html! {<a><i class="fa-solid fa-rectangle-history"></i>{"History"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Share"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Export google docs"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Export dox"}</a>},
        html! {<a><i class="fa-solid fa-file-pdf"></i>{"Export pdf"}</a>},
        html! {<a><i class="fa-solid fa-right-from-bracket"></i>{"Delete"}</a>},
    ];

    html! {
    <>
        <span {onmouseup} class="btn">
            <i class="fa-solid fa-ellipsis-vertical"></i>
        </span>


    </>
    }
}
