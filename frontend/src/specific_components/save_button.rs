use serde::{Deserialize, Serialize};
use shared::log;
use shared::schema::EditorChange;
use std::collections::VecDeque;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, DragEvent, Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::prelude::*;

// use shared::{invoke, log};

use crate::components::PopOverMenu;
use crate::router::Route;
use crate::specific_components::file_data::UseChangeHandle;
use crate::utils::DeviceInfo;
use crate::*;

#[derive(PartialEq, Properties)]
pub struct DownloadProps {
    // pub id: u64,
}

#[function_component]
pub fn SaveButton(props: &DownloadProps) -> Html {
    let (device, _) = use_store::<DeviceInfo>();
    let (changes, dispatch) = use_store::<UseChangeHandle>();
    let is_saved = changes.changes.is_empty();

    let onclick = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: MouseEvent| {
            let doc = window().unwrap_throw().document().unwrap_throw();
            let editor = doc.query_selector(".text_editor");
            let mut editables = window().unwrap_throw().document().unwrap_throw().query_selector_all("[contenteditable=true]");
            if let Some(editor) = editor.clone().unwrap() {
                editables = editor.query_selector_all("[contenteditable=true]");

                // prevent user to edit while saving
                if let Ok(editables) = editables.clone() {
                    for i in 0..editables.length() {
                        let editable = editables.get(i).unwrap();
                        editable
                            .dyn_ref::<Element>()
                            .unwrap()
                            .set_attribute("contenteditable", "false")
                            .unwrap();
                    }
                }
            }

            let target: Element = e.target_unchecked_into();
            let _ = target.class_list().add_1("loader").unwrap();
            let changes = changes.changes.clone();
            let dispatch = dispatch.clone();
            spawn_local(async move {
                let input_data = serde_json::to_string(&changes).unwrap_throw();
                // log!(&input_data);
                let res =
                    backend::call_ic("group_update".to_string(), input_data.to_string()).await;
                // log!("group is saved");
                // log!(&res);
                // if res.is_ok() {
                let empty_data: VecDeque<EditorChange> = VecDeque::new();
                let _ = dispatch.reduce_mut(|state| state.changes = empty_data);
                // }

                let _ = target.class_list().remove_1("loader");
                if let Some(editor) = editor.unwrap() {

                    // allow user to edit after save
                    if let Ok(editables) = editables.clone() {
                        for i in 0..editables.length() {
                            let editable = editables.get(i).unwrap();
                            editable
                                .dyn_ref::<Element>()
                                .unwrap()
                                .set_attribute("contenteditable", "true")
                                .unwrap();
                        }
                    }
                };
            });
        })
    };

    let mut save_mark = html! {
    <span class=" btn" >
        <i style="color: tomato" class="fa-solid fa-x"/>
        {"Save"}
    </span>
    };
    if is_saved {
        save_mark = html! {<span class=" btn"   >
            <i style="color: lightgreen" class="fa-solid fa-check"/>
            {"Saved"}
        </span>}
    }
    if device.is_web {
        return html! {
                <span {onclick}>
            {save_mark}
                </span>
        };
    }

    return html! {<></>};
}
