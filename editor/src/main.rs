extern crate web_sys;

use shared::schema::{EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode};
use shared::tree::{Tree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::Element;
use app::Editor;

mod app;
pub(crate) mod app_components;
pub(crate) mod components;
mod render;
mod plugins;
mod backend;
mod utils;

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
    let mut vertices: HashMap<Id, EditorElement> = HashMap::new();
    let mut adjacency: HashMap<Id, Vec<Id>> = HashMap::new();
    vertices.insert(Uuid::new_v4().into(), EditorElement {
        id: Uuid::new_v4().into(),
        tag: Some("h1".to_string()), // if tag.is_none() => doc.create_html_element("p")
        text: "hello world".to_string(),
        attrs: HashMap::new(),
    });
    adjacency.insert(id, vec![Uuid::new_v4().into(), Uuid::new_v4().into()]);  //TODO what is this?
    // TODO panicked at 'called `Option::unwrap()` on a `None` value', editor/src/render.rs:37:38

    let tree: ElementTree = ElementTree {
        id: Default::default(),
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