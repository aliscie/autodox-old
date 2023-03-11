// use crate::editor_components::EditorComponent;
use editor::EditorComponent;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, Range, window};
use yew::*;
use editor::Editor;
// use app::Editor;
// pub use frontend::dummy_data::*;
use shared::id::Id;
use shared::schema::{
    EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode,
};
use shared::tree::Tree;

use shared::schema::EditorChange;


use shared::*;

use shared::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserverInit, MutationRecord, Node};
use yew::prelude::*;
use yew::{function_component, html};

fn get_example_table() -> (Vec<EditorElement>, HashMap<Id, Vec<Id>>) {
    let root_table = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("table".to_string()),
    };
    let heading = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
    };
    let company = EditorElement {
        id: Id::new(),
        content: "Company".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
    };
    let table_body = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
    };
    let table_row = EditorElement {
        id: Id::new(),
        content: "Alfreds Futterkiste".to_string(),
        attrs: HashMap::new(),
        tag: Some("td".to_string()),
    };
    let adjacency_list = HashMap::from([
        (
            root_table.id,
            vec![heading.id, table_body.id, table_body.id],
        ),
        (heading.id, vec![company.id, company.id, company.id]),
        (
            table_body.id,
            vec![table_row.id, table_row.id, table_row.id],
        ),
    ]);
    return (
        vec![root_table, heading, company, table_body, table_row],
        adjacency_list,
    );
}


pub fn generate_dummy() -> ElementTree {
    let mut default_element_tree = ElementTree::default();
    let root = default_element_tree.elements.root.unwrap();
    let id: Id = Uuid::new_v4().into();
    let new_element = EditorElement {
        id: Uuid::new_v4().into(),
        content: "this is a bold and italic".to_string(),
        tag: Some("i".to_string()),
        attrs: HashMap::new(),
    };
    default_element_tree.elements.push_children(
        root,
        id.clone(),
        EditorElement {
            id,
            content: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::from([("style".to_string(), "color: tomato;".to_string())]),
            // TODO children: [new_element, ], I need the be an html child of the bold element
        },
    );

    let id: Id = Uuid::new_v4().into();
    default_element_tree.elements.push_children(
        root,
        id,
        EditorElement::new(id, r#"Lorem Ipsum is simply dummy text of the printing and typesetting
            industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s,
            when an unknown printer took a galley of type and scrambled it to make a type specimen book.
            It has survived not only five centuries, but also the leap into electronic typesetting,
            remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset
            sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."#.to_string(), HashMap::new()),
    );
    // here we rander all type of elements in order to test them.
    let table = get_example_table();
    for element in table.0 {
        default_element_tree.elements.push_children(root, element.id, element);
    }
    default_element_tree
}

fn onchange(e: EditorChange) {
    match e {
        EditorChange::Update(update_data) => {}
        EditorChange::Create(x) => {}
        EditorChange::Delete(data) => {}
    };
}

use editor::plugins::{CommandItems, DropDownItem, EditorInsert, EditorToolbar};

#[function_component]
pub fn DummyEditor() -> Html {
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
             element_tree={Rc::new(element_tree)}
             onchange = { Callback::from(onchange)}
    >
      </Editor<EditorComponent>>
     }
}

// fn main() {
//     yew::Renderer::<App>::new().render();
// }
