// editor/src/insertion_component
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
   let new_element = EditorElement {
       id: Uuid::new_v4().into(),
       text: "this is a bold and italic".to_string(),
       tag: Some("i".to_string()),
       attrs: HashMap::new(),
   };
   default_element_tree.elements.push_children(
       root,
       id.clone(),
       EditorElement {
           id,
           text: "this is a bold text".to_string(),
           tag: Some("b".to_string()),
           attrs: HashMap::from([("style".to_string(), "color: tomato;".to_string())]),
           // TODO children: [new_element, ], I need the be an html child of the bold element
       },
   );


   let id: Id = Uuid::new_v4().into();
   default_element_tree.elements.push_children(
       root,
       id,
       EditorElement::new(id, r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."#.to_string(), HashMap::new()),
   );
   let element_tree: Rc<RefCell<ElementTree>> =
       Rc::new(RefCell::new(default_element_tree.clone()));
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


