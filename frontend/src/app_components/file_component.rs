use serde::{Deserialize, Serialize};
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
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
    pub id: u64,
}


#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let display: UseStateHandle<bool> = use_state(|| false);

    let caret = use_state(|| "".to_string());
    let id = props.id.clone().to_string();

    let _display = display.clone();
    let onmousedown: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _e.which() == 3 {
            _display.set(true);
        }
    });

    let _caret = caret.clone();
    let toggle_caret = {
        move |_e: MouseEvent| {
            if _caret.len() == 0 {
                _caret.set("caret-down".to_string())
            } else {
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

    let ondrop: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let elem = _e.target_unchecked_into::<Element>();
        log_1(&format!("dragged_id {:?}", elem).into());
        // elem.set_attribute("style", "background: none");
    });


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
        <div>
            // if firts_file{
            //     <div
            //         ondrop={ondrop_b}
            //         ondragenter={ondragenter_b}
            //         ondragleave={ondragleave_b}
            //         style="width: 100%; height: 5px; background:red; opacity:0;"/>
            // }
            <div style="position: relative; width:100%">
                    {if props.class.contains("caret"){
                        html!{<button class={format!("{} crate_button",(*caret))} onmouseup={toggle_caret} onclick = { props.onclick.clone() } ><span class={format!("caret {}",(*caret).clone())}>{"➤"}</span></button>}
                    } else{ html!{} }
                    }

                    <li
                        {ondrop}
                        {ondragenter}
                        {ondragleave}
                        {ondragstart}
                        {ondragend}
                        {onmousedown}
                        onclick={props.onclickfile.clone()}
                        draggable="true"
                        class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(), (*is_enter).clone(), "")}
                        style="margin-left: 30px; flex: 1 1 auto; white-space: nowrap; min-width: 0px; overflow: hidden; text-overflow: ellipsis; display: flex; align-items: center;"
                        >
                        <div class="notranslate" style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                            {props.name.clone()}
                        </div>
                        <span class="create_file" >{"+"}</span>
                    </li>

            </div>

        <div
                ondrop={ondrop_b}
                ondragenter={ondragenter_b}
                ondragleave={ondragleave_b}
                class="drag_under"
                style="width: 100%; height: 20px; background: red; opacity: 0"/>


        <Menu {display}/>
    </div>
    }
}
