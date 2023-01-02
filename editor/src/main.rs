extern crate web_sys;

mod editor_components;

use app::Editor;
use shared::schema::{
    EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode,
};
use shared::tree::Tree;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;

mod app;
mod backend;
pub(crate) mod components;
mod plugins;
mod render;
pub(crate) mod spesific_components;

use crate::app::EditorChange;
use shared::id::Id;
use yew::*;

fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        // log!(&e)
    })
}

#[function_component]
pub fn App() -> Html {
    let mut default_element_tree = ElementTree::default();
    let root = default_element_tree.elements.root.unwrap();
    let id: Id = Uuid::new_v4().into();
    default_element_tree.elements.push_children(
        root,
        id.clone(),
        EditorElement::new(
            id,
            "bold text".to_string(),
            HashMap::from([("style".to_string(), "font-weight: bold;".to_string())]),
        ),
    );
    let id: Id = Uuid::new_v4().into();
    default_element_tree.elements.push_children(
        root,
        id,
        EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
    );
    let element_tree: Rc<RefCell<ElementTree>> =
        Rc::new(RefCell::new(default_element_tree.clone()));
    html! {
        <div>
        <h1>{"Text editor test"}</h1>
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
