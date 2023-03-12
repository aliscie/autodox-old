use std::cell::RefCell;
use std::collections::HashMap;
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
use yew::{function_component, html};

fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        match e {
            EditorChange::Update(x) => {
                let update_data = x.clone();
                if let Some(element) = element_tree
                    .as_ref()
                    .borrow_mut()
                    .elements
                    .vertices
                    .get_mut(&x.id)
                {
                    if let Some(text) = x.text {
                        element.text = text;
                    }
                    if let Some(attrs) = x.attrs {
                        element.attrs = attrs;
                    }
                }
            }
            EditorChange::Create(x) => {
                let create_data = x.clone();
                element_tree.as_ref().borrow_mut().elements.push_children(
                    x.parent_id.clone(),
                    x.id.clone(),
                    x.clone().into(),
                );
                if let Some(prev_element_id) = x.prev_element_id {
                    let mut element_tree = element_tree.as_ref().borrow_mut();
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
            }
            EditorChange::Delete(data) => {
                element_tree.as_ref().borrow_mut().elements.remove(&data.id);
            }
        };
    })
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
             element_tree={element_tree.clone()}
             onchange = { onchange(element_tree.clone())}
    >
      </Editor>
     }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
