use editor::Editor;
use shared::schema::{Attrs, EditorElement, ElementTree, FileDirectory};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[function_component(FileData)]
pub fn file_data(props: &Props) -> Html {
    let dispatch = Dispatch::<FileDirectory>::new();
    let mut r: ElementTree = ElementTree::default();
    let root = r.elements.root.unwrap();
    let id = Uuid::new_v4();
    r.elements.push_children(
        root,
        id.clone(),
        EditorElement::new(
            id,
            "bold text".to_string(),
            HashMap::from([(Attrs::Style, "font-weight: bold;".to_string())]),
        ),
    );
    let id = Uuid::new_v4();
    r.elements.push_children(
        root,
        id,
        EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
    );
    let element_tree = Rc::new(RefCell::new(r));
    match dispatch.get().files.vertices.get(&props.id) {
        Some(x) => html! {
            <>
                <Editor title = { x.name.clone() } element_tree = { element_tree }/>
            </>
        },
        None => html! { <div> {"File not found!"} </div>},
    }
}
