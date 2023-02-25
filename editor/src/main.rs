use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Element, Range};
use yew::*;

use app::Editor;
pub use dummy_data::*;
use shared::id::Id;
use shared::schema::{
    EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode,
};
use shared::tree::Tree;

use shared::schema::EditorChange;

mod editor_components;
mod utils;

mod app;
mod backend;
pub(crate) mod components;
mod dummy_data;
mod handle_mutation;
mod insertion_closures;
mod plugins;
mod render;
pub(crate) mod spesific_components;

use shared::*;

use shared::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserverInit, MutationRecord, Node};
use yew::prelude::*;
use yew::{function_component, html};

fn onchange((element_tree, e): (UseStateHandle<ElementTree>, EditorChange)) {
    match e {
        EditorChange::Update(update_data) => {
            element_tree.set({
                let mut element_tree = element_tree.deref().clone();
                if let Some(element) = element_tree.elements.vertices.get_mut(&update_data.id) {
                    if let Some(text) = update_data.text {
                        element.text = text;
                    }
                    if let Some(attrs) = update_data.attrs {
                        element.attrs = attrs;
                    }
                }
                element_tree
            });
        }
        EditorChange::Create(x) => {
            element_tree.set({
                let mut element_tree = element_tree.deref().clone();
                element_tree.elements.push_children(
                    x.parent_id.clone(),
                    x.id.clone(),
                    x.clone().into(),
                );
                if let Some(prev_element_id) = x.prev_element_id {
                    let children_list_of_parent_element = element_tree
                        .elements
                        .adjacency
                        .get_mut(&x.parent_id)
                        .unwrap();
                    let index_of_prev_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == x.id)
                        .unwrap();
                    let index_of_last_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == x.id)
                        .unwrap();
                    children_list_of_parent_element
                        .swap(index_of_last_element, index_of_prev_element);
                }
                element_tree
            });
        }
        EditorChange::Delete(data) => {
            element_tree.set({
                let mut element_tree = element_tree.deref().clone();
                element_tree.elements.remove(&data.id);
                element_tree
            });
        }
    };
}

use crate::plugins::{CommandItems, DropDownItem, EditorInsert, EditorToolbar};

#[function_component]
pub fn App() -> Html {
    let element_tree = generate_dummy();

    let action: Callback<String> = Callback::from(move |e: String| {
        // log!(e.clone());
        // onchange.emit(EditorChange::Update(EditorElementUpdate {
        //     id: element_tree.as_ref().borrow().elements.root.unwrap(),
        //     text_format: Some(format),
        //     ..Default::default()
        // }));
    });
    html! {
    <Editor
             title = {"untitled".to_string()}
             element_tree={element_tree}
             onchange = { Callback::from(onchange)}
    >
      </Editor>
     }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
