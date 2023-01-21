// editor/src/insertion_component
extern crate web_sys;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;
use web_sys::Element;
use yew::*;

use app::Editor;
pub use dummy_data::*;
use shared::id::Id;
use shared::schema::{
    EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode,
};
use shared::tree::Tree;

use crate::app::EditorChange;

mod editor_components;

mod insertion_closures;
mod app;
mod backend;
pub(crate) mod components;
mod plugins;
mod render;
pub(crate) mod spesific_components;
mod dummy_data;

fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        // log!(&e)
    })
}

#[function_component]
pub fn App() -> Html {
    let element_tree = generate_dummy();
    html! {
       <div>
       <Editor
           title = {"untitled".to_string()}
           element_tree={element_tree.clone()}
           onchange = { onchange(element_tree.clone())}
      />
       </div>
   }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


