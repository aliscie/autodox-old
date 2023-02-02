use crate::pages::PagesRoute;
use crate::{backend, components::PopOverMenu, router::Route};
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

#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
    pub id: Id,
}

#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    // HasMap
    // {
    // type:"dropover", // or dropunder or dropbellow,
    // dragged_id: uuid,
    // target_id: uuid
    // }
    //let drop_data = use_state(|| "".to_string());
    //let is_drag_over = use_state(|| "".to_string());
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    let is_drag_under = use_state(|| "".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or_default();

    let caret = use_toggle("", "caret-down");
    let id = props.id.clone();

    let toggle_caret = {
        let caret = caret.clone();
        move |_e: MouseEvent| {
            caret.toggle();
        }
    };

    let ondragstart: Callback<DragEvent> = {
        let is_dragged = is_dragged.clone();
        let id = id.clone();

        Callback::from(move |e: DragEvent| {
            e.data_transfer()
                .unwrap()
                .set_data("dragged_item", &id.to_string())
                .unwrap();
            is_dragged.set("dragged".to_string())
        })
    };

    let ondragend: Callback<DragEvent> = {
        let is_dragged = is_dragged.clone();
        let is_enter = is_enter.clone();
        Callback::from(move |_e: DragEvent| {
            is_dragged.set("".to_string());
            is_enter.set("".to_string());
        })
    };

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_enter.set("dragging_over".to_string());
        }
    });

    let _is_enter = is_enter.clone();
    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("".to_string());
    });

    // let ondrop: Callback<DragEvent> = {
    //     let id = id.clone();
    //     let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
    //     Callback::from(move |e: DragEvent| {
    //         e.prevent_default();
    //         let curr: Element = e.target_unchecked_into();
    //         let _ = curr.class_list().toggle("dragging_over");
    //         let dragged = e.data_transfer().unwrap().get_data("dragged_item").unwrap();
    //         let id = id.clone();
    //         let mut old_parent_id: Id = Uuid::new_v4().into();
    //         let dragged_uuid = Uuid::parse_str(dragged.as_str()).map(Id::from).unwrap();
    //         for (i, value) in &_dispatch_file_directory.get().files.adjacency {
    //             if value.contains(&dragged_uuid) {
    //                 old_parent_id = *i;
    //                 break;
    //             }
    //         }
    //         crate::backend::change_directory(id.to_string(), dragged, old_parent_id.to_string());
    //     })
    // };

    let _id = id.clone();
    let ondrop: Callback<DragEvent> =
        dispatch_file_directory.reduce_mut_future_callback_with(move |state, _e: DragEvent| {
            _e.prevent_default();
            let curr: Element = _e.target_unchecked_into();
            curr.class_list().toggle("dragging_over");
            let dragged = _e
                .data_transfer()
                .unwrap()
                .get_data("dragged_item")
                .unwrap();
            Box::pin(async move {
                let mut old_parent_id: Id = Uuid::new_v4().into();
                let dragged_uuid = Uuid::parse_str(dragged.as_str()).map(Id::from).unwrap();
                for (i, value) in &state.files.adjacency {
                    if value.contains(&dragged_uuid) {
                        old_parent_id = *i;
                        break;
                    }
                }
                crate::backend::change_directory(
                    _id.clone().to_string(),
                    dragged.clone(),
                    old_parent_id.clone().to_string(),
                );
                // Update file directory in the frontend
                let child_id = Id::from_str(&dragged.clone()).unwrap();
                let old_adjacency = state.files.adjacency.get_mut(&old_parent_id).unwrap();
                if old_adjacency.len() > 0 {
                    let file_index = old_adjacency
                        .iter()
                        .position(|x| *x == child_id.clone())
                        .unwrap();
                    old_adjacency.remove(file_index);
                }
                let mut new_adjacency = state.files.adjacency.entry(_id.clone()).or_default();
                new_adjacency.push(child_id.clone());
            })
        });

    let _is_drag_under = is_drag_under.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_drag_under.set("height: 20px; opacity:1;".to_string());
        }
    });

    let ondragover: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
    });

    let _is_drag_under = is_drag_under.clone();
    let _id = id.clone();
    let ondrop_under: Callback<DragEvent> = Callback::from(move |e: DragEvent| {
        e.prevent_default();
        let curr: Element = e.target_unchecked_into();
        let _ = curr.set_attribute("style", "height: 3px; opacity:0;");

        let _dragged = e.data_transfer().unwrap().get_data("dragged_item");
    });

    let _id = id.clone();
    let on_create_file: Callback<MouseEvent> = dispatch_file_directory
        .reduce_mut_future_callback_with(move |state, _e: MouseEvent| {
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

    let _id = id.clone();
    let onkeydown: Callback<KeyboardEvent> = dispatch_file_directory
        .reduce_mut_future_callback_with(move |state, _e: KeyboardEvent| {
            Box::pin(async move {
                let input: HtmlInputElement = _e.target_unchecked_into();
                let value: String = input.inner_text();

                if _e.key() == "Enter" {
                    _e.prevent_default();
                };

                if _e.key() != "Enter" {
                    input.class_list().remove_1("tool").unwrap();
                };

                let clone_id = _id.clone();

                if _e.key() == "Enter" && value.is_empty() {
                    input.class_list().add_1("tool").unwrap();
                } else if _e.key() == "Enter" {
                    let res = backend::rename_file(_id.clone(), value.clone()).await;
                    if (res.is_ok()) {
                        state.files.vertices.get_mut(&_id).unwrap().name = value;
                    }
                }
            })
        });

    let ondelete = {
        let id = id.clone();
        let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
        let mut parent_id = Id::default();
        for (parent, child_id) in &_dispatch_file_directory.get().files.adjacency {
            if child_id.contains(&id) {
                parent_id = *parent;
            }
        }
        let delete_file_node = FileNodeDelete {
            id,
            parent_id,
            tree_id: _dispatch_file_directory.get().id,
        };
        let file_id = id.clone();
        let _navigator = navigator.clone();
        _dispatch_file_directory.reduce_mut_future_callback(move |state| {
            match route {
                // the current file is in use navigate to home!
                Route::File { id } => {
                    if file_id == id {
                        _navigator.push(&Route::Home);
                    }
                }
                _ => {}
            }
            Box::pin(async move {
                let result = crate::backend::delete_file(delete_file_node).await;

                log!(result);
                state.files.remove(&file_id);
            })
        })
    };

    let _is_drag_under = is_drag_under.clone();
    let ondragleave_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_drag_under.set("".to_string());
    });
    let oncontextmenu = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            position.set(Some(e));
        })
    };

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

    let on_permission = {
        let _navigator = navigator.clone();
        let _id = props.id.clone();
        move |_| _navigator.push(&PagesRoute::Permission { id: _id })
    };

    html! {
        <div
        class={css_file_macro!("file_component.css")}
        >
        // TODO
        //  {if is_first_file {
        //         html!{
        //         <div
        //            ondrop={ondrop_above}
        //            ondragenter={ondragenter_above}
        //            ondragleave={ondragleave_above}
        //            class="drag_under"/>
        //         }
        //  }}

        <div {oncontextmenu} style="position: relative; width:100%; display: block;">
           if props.class.contains("caret"){
               <button class={format!("{} crate_button",(*caret))}
               onmouseup={toggle_caret}
               onclick = { props.onclick.clone() }><i class="fa-solid fa-caret-right"></i></button>
           }
           <li
           ondragover={ondragover.clone()}
           {ondrop}
           {ondragenter}
           {ondragleave}
           {ondragstart}
           {ondragend}
           id = { id.to_string()}
           onclick={props.onclickfile.clone()}
           draggable="true"
           class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(),(*is_enter).clone(), "")}
          >
           {props.name.clone()}
           </li>
           <i style="height:100%" class="btn create-file fa-solid fa-plus"
           onclick={on_create_file}
           ></i>
        </div>

            // <div
            // ondragover={ondragover.clone()}
            // style={format!("{}",(*is_drag_under).clone())}
            // ondrop={ondrop_under}
            // ondragenter={ondragenter_under}
            // ondragleave={ondragleave_under}
            // class="drag_under"/>

           <PopOverMenu
            click_on={Some(true)}
           position = {position.clone()}
           items={vec![
           html! {<a><span
                {onkeydown}
                contenteditable="true"
                data-tip="File must have at least 1 character."
                autofocus=true placeholder="rename..">{props.name.clone()}</span></a>},
           html! {<a><i class="fa-solid fa-upload"/>{"Share"}</a>},
           html! {<a onclick={on_permission}><i class="fa-solid fa-eye"/>{"Permissions"}</a>},
           html! {<a onclick = {ondelete}><i class="fa-solid fa-trash"/>{"Delete"}</a>},
           html! {<a><i class="fa-brands fa-medium"></i>{"Category"}</a>},
           ]}
          />

        </div>

    }
}
