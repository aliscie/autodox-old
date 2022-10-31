use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::router::Route;
use crate::router::*;

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
    <span>
        <span
           class="hovering file_component">
        {"parent file test 1"}
        </span>
        {"/"}
        <span class="hovering file_component">
           <Switch
           <Route>
           render={Switch::render(switch)} />
        </span>
        <span class="btn" style="width: 35px"><i class="fa-solid fa-share"></i></span>

    </span>
    }
}
