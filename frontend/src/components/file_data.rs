use editor::Editor;
use wasm_bindgen_futures::spawn_local;
use shared::log;
use shared::schema::{Attrs, EditorElement, ElementTree, FileDirectory};
use web_sys::console;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;
use editor::EditorChange;

use crate::backend::update_element;

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
    let onchange: Callback<EditorChange> = Callback::from(move |x| {
        console::log_1(&format!("got changes in frontend crate : {:?}", x).into());
        match x {
            EditorChange::Update(data) => { 
                spawn_local(async move {
                    log!(update_element(data).await);
                })
            },
            _ => log!("Not implemented this yet!"),
        }
    });
    match dispatch.get().files.vertices.get(&props.id) {
        Some(x) => {
            if x.element_tree.is_none() {     
                // creating new element tree!

            }
            let element_tree = Rc::new(RefCell::new(r));
            html! {
            <>
                <Editor title = { x.name.clone() } element_tree = { element_tree } { onchange } />
            </>
        }},
        None => html! { <div> {"File not found!"} </div>},
    }
}
