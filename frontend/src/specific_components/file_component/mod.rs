use crate::pages::PagesRoute;
use crate::{backend, components::PopOverMenu, router::Route};
use shared::schema::FileMode;
use shared::*;
use shared::{
    id::Id,
    log,
    schema::{FileDirectory, FileNode, FileNodeDelete},
};
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;
use yew_hooks::use_toggle;
use yew_router::prelude::{use_navigator, use_route};
use yewdux::prelude::*;
mod handle_menu_items;
use handle_menu_items::use_file_items;
mod draggable_component;
use draggable_component::DragComponent;

#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub class: String,
    pub file_node: FileNode,
}

#[function_component]
pub fn FileComponent(props: &FileComponentProps) -> Html {
    let menu_items = use_file_items(props.clone());
    // HasMap
    // {
    // type:"dropover", // or dropunder or dropbellow,
    // dragged_id: uuid,
    // target_id: uuid
    // }
    //let drop_data = use_state(|| "".to_string());
    //let is_drag_over = use_state(|| "".to_string());
    let (fd_rc, fd_dispatch) = use_store::<FileDirectory>();
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let caret = use_toggle("", "caret-down");
    let id = props.file_node.id.clone();

    let toggle_caret = {
        let caret = caret.clone();
        move |_e: MouseEvent| {
            caret.toggle();
        }
    };


    let _id = id.clone();
    let on_create_file: Callback<MouseEvent> =
        fd_dispatch.reduce_mut_future_callback_with(move |state, _e: MouseEvent| {
            Box::pin(async move {
                let mut file = FileNode::default();
                let file_name = "new file".to_string();
                file.name = file_name.clone();
                let x =
                    crate::backend::create_file(state.id, _id.clone(), file_name, file.id).await;
                if x.is_ok() {
                    state.files.push_children(_id.clone(), file.id, file);
                }
            })
        });






    let oncontextmenu = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            position.set(Some(e));
        })
    };



    let mut style = "color: tomato;";
    if props.file_node.file_mode == FileMode::Public {
        style = "color: lightgreen;";
    };

    html! {
        <div
        {style}
        class={css_file_macro!("file_component.css")}
        >


        <div {oncontextmenu} style="position: relative; width:100%; display: block;">
           if props.class.contains("caret"){
               <button class={format!("{} crate_button",(*caret))}
               onmouseup={toggle_caret}
               onclick = { props.onclick.clone() }><i class="fa-solid fa-caret-right"></i></button>
           }

            <span onclick={props.onclickfile.clone()}>
                <DragComponent id={id.clone()}>{props.file_node.name.clone()}</DragComponent>
            </span>


           <i style="height:100%" class="btn create-file fa-solid fa-plus"
           onclick={on_create_file}
           ></i>
        </div>



           <PopOverMenu
            click_on={Some(true)}
           position = {position.clone()}
           items={menu_items.clone()}
          />

        </div>

    }
}
