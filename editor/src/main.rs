use crate::editor_components::EditorComponent;
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

fn onchange(e: EditorChange) {
    match e {
        EditorChange::Update(update_data) => {}
        EditorChange::Create(x) => {}
        EditorChange::Delete(data) => {}
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
    <Editor<EditorComponent>
             title = {"untitled".to_string()}
             element_tree={element_tree}
             onchange = { Callback::from(onchange)}
    >
      </Editor<EditorComponent>>
     }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
