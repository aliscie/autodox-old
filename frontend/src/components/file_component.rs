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
    log_1(&format!("render {:?}", "render only once").into()); // TODO why this rerender for every click .

    let display = use_state(|| "display: none;".to_string());
    let onmousedown = {
        let display = display.clone();
        move |_e: MouseEvent| {
            if _e.which() == 3 {
                display.set("display: block".to_string());
            }
        }
    };


    let ondragstart: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        // TODO make
        //  curr.style.add("opacity", "0.6")
        //  in order to not to replace the old styles
        curr.set_attribute("style", "opacity: 0.4");
    });

    let ondragend: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        // TODO make
        //  curr.style.remove("opacity")
        //  in order to keep the old styles
        curr.set_attribute("style", "opacity: 1");
    });

    let ondragover: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
    });

    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "background: lightblue");
    });


    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "background: none");
    });

    let ondragend: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "background: none");
    });

    let ondrop: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "background: none"); // TODO this is not working?
    });

    // TODO Be carefully previously the app freeze when I uncomment this?
    //  it freeze after clicking 3 time son a file.

    let doc = window().unwrap_throw().document().unwrap();
    let x = display.clone();
    let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if (*x).clone() == "display: block".to_string() {
            &x.set("display: none".to_string());
        }
    }) as Box<dyn FnMut(_)>);

    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("click", &click_away_handler.as_ref().unchecked_ref());
    click_away_handler.forget();


    html! {
        <>
            <li
                {ondragover}
                {ondrop}
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
