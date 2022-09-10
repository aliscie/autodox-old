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
    pub id: u64,
}


#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    let doc = window().unwrap_throw().document().unwrap();
    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());

    let display = use_state(|| "display: none;".to_string());
    let caret = use_state(|| "".to_string());
    let id = props.id.clone().to_string();

    let onmousedown = {
        let display = display.clone();
        move |_e: MouseEvent| {
            if _e.which() == 3 {
                display.set("display: block".to_string());
            }
        }
    };
    let _caret = caret.clone();
    let toggle_caret = {
        let display = display.clone();
        move |_e: MouseEvent| {
            if _caret.len()==0{
                _caret.set("caret-down".to_string())
            } else{
                _caret.set("".to_string())
            }
        }
    };

    let _is_dragged = is_dragged.clone();
    let ondragstart: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_dragged.set("dragged".to_string())
    });

    let _is_dragged = is_dragged.clone();
    let ondragend: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_dragged.set("".to_string())
    });

    let _is_enter = is_enter.clone();
    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("dragging_over".to_string());
    });

    let _is_enter = is_enter.clone();
    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("".to_string());
    });

    let _doc = &doc.clone();
    let ondrop: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let elem = _e.target_unchecked_into::<Element>();
        log_1(&format!("dragged_id {:?}", elem).into());
        // elem.set_attribute("style", "background: none");
    });

    // TODO Be carefully previously the app freeze when I uncomment this?
    //  it freeze after clicking 3 time son a file.


    let x = display.clone();
    let click_away_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        if (*x).clone() == "display: block".to_string() {
            &x.set("display: none".to_string());
        }
    }) as Box<dyn FnMut(_)>);

    let _ = &doc.query_selector("#app").unwrap().unwrap().add_event_listener_with_callback("click", &click_away_handler.as_ref().unchecked_ref());
    click_away_handler.forget();


    let ondragenter_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr = _e.target_unchecked_into::<Element>();
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "width: 100%; height: 20px; background:lightblue;");
    });


    let ondrop_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let curr = _e.target_unchecked_into::<Element>();
        curr.set_attribute("style", "width: 100%; height: 5px; background:red; opacity:0;");
    });


    let ondragleave_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "width: 100%; height: 5px; background:red; opacity:0;");
    });

    html! {
        <div class={format!("active {} {} {}",(*is_dragged).clone(), (*is_enter).clone(), props.class.clone())}  style="position: relative; width:100%">
            // if firts_file{
            //     <div
            //         ondrop={ondrop_b}
            //         ondragenter={ondragenter_b}
            //         ondragleave={ondragleave_b}
            //         style="width: 100%; height: 5px; background:red; opacity:0;"/>
            // }

            {if props.class.contains("caret"){
                html!{<button class="crate_button" onmouseup={toggle_caret} onclick = { props.onclick.clone() } ><span class={format!("caret {}",(*caret).clone())}>{"➤"}</span></button>}
            } else{ html!{} }
            }

            <li
                {ondrop}
                {ondragenter}
                {ondragleave}
                {ondragstart}
                {ondragend}
                {onmousedown}
                draggable="true"

                style="margin-left: 30px; flex: 1 1 auto; white-space: nowrap; min-width: 0px; overflow: hidden; text-overflow: ellipsis; display: flex; align-items: center;"
                >
                <div class="notranslate" style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                    {props.name.clone()}
                </div>
            </li>
        <button style="right:0; top:50%; position: absolute; -webkit-transform: translateY(-50%)" >{"+"}</button>

        <div
                ondrop={ondrop_b}
                ondragenter={ondragenter_b}
                ondragleave={ondragleave_b}
                class="drag_under"
                style="width: 100%; height: 20px;"/>

        <div
        style={(*display).clone()}
        class={"dropdown-content"}>
        <a href="#"><i class="gg-software-upload"/>{"Share."}</a>
        <a href="#"><i class="gg-trash"/>{"Delete."}</a>
        <a href="#"><i class="gg-eye-alt"></i>{"Who can see."}</a>
        </div>
    </div>
    }
}
