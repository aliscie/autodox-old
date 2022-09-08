// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use serde::{Deserialize, Serialize};
use yew::{html, Html};
use web_sys;
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
            if _e.which() == 3 {
                display.set("display: block".to_string());
            }
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
    // TODO why the app free when I uncomment this?
    // let doc = window().unwrap_throw().document().unwrap();
    // let x = display.clone();
    // let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    //     x.set("display: none".to_string());
    // }) as Box<dyn FnMut(_)>);
    // let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("click", &click_away_handler.as_ref().unchecked_ref());
    // click_away_handler.forget();

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
        style={(*display).clone()}
        class={"dropdown-content"}>
        <a href="#">{"☁"}</a>
        <a href="#">{"🗑"}</a>
        <a href="#">{"👁"}</a>
        </div>
    </>
    }
}
