extern crate web_sys;

mod editor_components;

use shared::schema::{EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode};
use shared::tree::{Tree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use app::Editor;

mod plugins;
mod app;
pub(crate) mod spesific_components;
pub(crate) mod components;
mod render;
mod backend;

use yew::*;
use shared::id::Id;
use crate::app::EditorChange;


fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        // log!(&e)
    })
}


#[function_component]
pub fn App() -> Html {
    let id: Id = Uuid::new_v4().into();
    let element_id: Id = Uuid::new_v4().into();
    let mut vertices: HashMap<Id, EditorElement> = HashMap::new();
    let mut adjacency: HashMap<Id, Vec<Id>> = HashMap::new();
    vertices.insert(id, EditorElement::new(
        element_id,
        "bold text".to_string(),
        HashMap::from([(
            "style".to_string(),
            "font-weight: bold;".to_string(),
        )]),
    ));

    adjacency.insert(id, vec![id]);  //TODO what is this?
    // TODO panicked at 'called `Option::unwrap()` on a `None` value', editor/src/render.rs:37:38

    let tree: ElementTree = ElementTree {
        id: id,
        elements: Tree {
            vertices,
            adjacency,
            root: Some(id),
        },
    };

    let element_tree: Rc<RefCell<ElementTree>> = Rc::new(RefCell::new(tree.clone()));
    html! {
    < >
        <h1>{"Text editor test"}</h1>
        <Editor
        title = {"untitled".to_string()}
        element_tree={element_tree.clone()}
        onchange = { onchange(element_tree.clone())}
        />
    < / >
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}