// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use serde::{Deserialize, Serialize};
use yew::{html, Html};
use web_sys::console::log_1;
use web_sys::{window, Element, MouseEvent, DragEvent};
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::{UnwrapThrowExt, JsCast};

use crate::router::Route;
use yewdux::prelude::*;


#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
}


#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    let display = use_state(|| "display: none;".to_string());
    let onmousedown = {
        let display = display.clone();
        move |_e: MouseEvent| {
            display.set("display: block".to_string());
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
            <li
                {ondragstart}
                {ondragend}
                {ondragenter}
                {ondragleave}
                {onmousedown}
                draggable="true"
                class={props.class.clone()}
                onclick = { props.onclick.clone() }
                style="flex: 1 1 auto; white-space: nowrap; min-width: 0px; overflow: hidden; text-overflow: ellipsis; display: flex; align-items: center;"
                >
                <div class="notranslate" style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                    {props.name.clone()}

                </div>
            </li>
        <div
        style = { (*display).clone() }
         class={"dropdown-content"}>
        <a href="#">{"☁"}</a>
        <a href="#">{"🗑"}</a>
        <a href="#">{"👁"}</a>
        </div>
    </>
    }
}
