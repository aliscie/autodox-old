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

use crate::components::{Menu};
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
    // HasMap
    // {
    // type:"dropover", // or dropunder or dropbellow,
    // dragged_id: uuid,
    // target_id: uuid
    // }
    let drop_data = use_state(|| "".to_string());
    let is_drag_over = use_state(|| "".to_string());
    let is_drag_under = use_state(|| "".to_string());


    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let caret = use_state(|| "".to_string());
    let id = props.id.clone().to_string();

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _e.which() == 3 {
            _position.set(
                Some(_e)
            );
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
    let _is_enter = is_enter.clone();
    let ondragend: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_dragged.set("".to_string());
        _is_enter.set("".to_string());
    });

    let _is_enter = is_enter.clone();
    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("dragging_over".to_string());
    });

    let _is_enter = is_enter.clone();
    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("".to_string());
    });

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let ondrop: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        // TODO
        //  get dragged item by datatransfer
        //  get dragged over item id props.id
        //  do the child transaction from parent to another parent
    });


    let _is_drag_under = is_drag_under.clone();
    let ondragenter_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_drag_under.set("height: 20px; opacity:1;".to_string());
    });

    let _is_drag_under = is_drag_under.clone();
    let ondrop_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        //TODO
        // let id = e.data_transfer_get("id");
        // let doc = window().unwrap_throw().document().unwrap();
        // let curr = query_selector(format!("#{}",id)).unwrap().unwrap();
        // curr.set_attribute("style", " height: 3px; opacity:0;");
    });

    let _is_drag_under = is_drag_under.clone();
    let ondragleave_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_drag_under.set("".to_string());
    });

    // let ondragenter_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("height: 20px; opacity:1;".to_string());
    // });

    // let ondragleave_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("".to_string());
    // });

    // let ondrop_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _e.prevent_default();
    //     _is_drag_above.set("".to_string());
    // });


    html! {
        <div>
        // TODO
        //  {if is_first_file {
        //         html!{
        //         <div
        //            ondrop={ondrop_above}
        //            ondragenter={ondragenter_above}
        //            ondragleave={ondragleave_above}
        //            class="drag_under" />
        //         }
        //  }}

        <div style="position: relative; width:100%; display: block;">
           {if props.class.contains("caret"){
           html!{<button class={format!("{} crate_button",(*caret))} onmouseup={toggle_caret} onclick = { props.onclick.clone() } ><i class="fa-solid fa-caret-right"></i></button>}
           } else{ html!{} }
           }
           <li
           {ondrop}
           {ondragenter}
           {ondragleave}
           {ondragstart}
           {ondragend}
           {onmouseup}
           id = { id }
           onclick={props.onclickfile.clone()}
           draggable="true"
           class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(),(*is_enter).clone(), "")}
           style="margin-left: 30px; min-width: 0px; align-items: center; height: 100%; display: block;"
           >
           {props.name.clone()}
           </li>
           <i class="btn create_file fa-solid fa-plus"></i>
        </div>

            <div
            style={format!("{}",(*is_drag_under).clone())}
            ondrop={ondrop_under}
            ondragenter={ondragenter_under}
            ondragleave={ondragleave_under}
            class="drag_under" />

           <Menu
           items={vec![
           html! {<><i class="fa-solid fa-signature"></i>{"Rename"}</>},
           html! {<><i class="fa-solid fa-upload"/>{"Share"}</>},
           html! {<><i class="fa-solid fa-eye"/>{"Permissions"}</>},
           html! {<><i class="fa-solid fa-trash"/>{"Delete"}</>},

           ]}
           event={position.clone()}
           />

        </div>

    }
}
