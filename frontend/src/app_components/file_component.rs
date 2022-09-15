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


    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let position: UseStateHandle<String> = use_state(|| "".to_string());

    let caret = use_state(|| "".to_string());
    let id = props.id.clone().to_string();

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _e.which() == 3 {
            _position.set(
                format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
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



    let ondragenter_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr = _e.target_unchecked_into::<Element>();
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "height: 20px; opacity:1;");
    });

    let ondrop_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let curr = _e.target_unchecked_into::<Element>();
        curr.set_attribute("style", " height: 5px; opacity:0;");
        // TODO call this here

        // TODO
        //  get dragged item by dataframes
        //  do reorder the item or move it from parent to parent
        //     cases
        //     drag from place to place
        //     drag from paren to another paren
        //     drag from root to a parent..
    });



    let ondragleave_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr = _e.target_unchecked_into::<Element>();
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "height: 20px; opacity:1;");
    });

    let ondrop_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let curr = _e.target_unchecked_into::<Element>();
        curr.set_attribute("style", " height: 5px; opacity:0;");
    });


    let ondragleave_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", " height: 5px; opacity:0;");
    });


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
           html!{<button class={format!("{} crate_button",(*caret))} onmouseup={toggle_caret} onclick = { props.onclick.clone() } ><span class={format!("caret {}",(*caret).clone())}>{"➤"}</span></button>}
           } else{ html!{} }
           }
           <li
           {ondrop}
           {ondragenter}
           {ondragleave}
           {ondragstart}
           {ondragend}
           {onmouseup}
           onclick={props.onclickfile.clone()}
           draggable="true"
           class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(),(*is_enter).clone(), "")}
           style="margin-left: 30px; min-width: 0px; align-items: center; height: 100%; display: block;"
           >
           {props.name.clone()}
           </li>
           <button class="create_file" >{"+"}</button>
        </div>

            <div
           ondrop={ondrop_under}
           ondragenter={ondragenter_under}
           ondragleave={ondragleave_under}
           class="drag_under" />

           <Menu
           items={vec![
           html! {<><i class="gg-software-upload"/>{"Share."}</>},
           html! {<><i class="gg-software-upload"/>{"Share."}</>}
           ]}
           position={position.clone()}
           />

        </div>

    }
}
