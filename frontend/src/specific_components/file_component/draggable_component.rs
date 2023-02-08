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

#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
    pub id: Id,
}

#[function_component]
pub fn DragComponent() -> Html {
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
        fd_dispatch.reduce_mut_future_callback_with(move |state, _e: DragEvent| {
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

    html! {
        <div>

        </div>

    }
}
