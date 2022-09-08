use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use web_sys::console::log_1;
use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::router::*;
use crate::router::Route;

#[function_component(CurrDirectory)]
pub fn curr_directory() -> Html {
    let display = use_state(|| "display: none;".to_string());
    let onmousedown = {
        let display = display.clone();
        move |e: MouseEvent| {
            // display.set("display: block".to_string());
        }
    };


    let ondragstart = {
        move |e: DragEvent| {
            // opacity:0.5
        }
    };

    let ondragend = {
        move |e: DragEvent| {
            // opacity:1
        }
    };

    let ondragenter = {
        move |e: DragEvent| {
            // background:lightblue
        }
    };


    let ondragleave = {
        move |e: DragEvent| {
            // background:none
        }
    };


    html! {
    <>
        <span
         class="hovering file_component">
        {"parent file"}
        </span>
        {"/"}
        <span class="hovering file_component">
        <Switch<Route> render={Switch::render(switch)} />
        </span>
    </>
    }
}
