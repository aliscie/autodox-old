use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use shared::{
    id::Id,
    log,
    schema::{EditorChange, EditorElement, EditorElementCreate, ElementTree},
};
use uuid::Uuid;
use web_sys::{window, HtmlElement, Range};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{plugins::DropDownItem, render::render};
use wasm_bindgen::JsCast;

pub fn on_slash_input(
    event: DropDownItem,
    range: Option<Range>,
    element_tree: &ElementTree,
) -> Option<(Id, Vec<EditorChange>)> {
    // if no focused element is found we are selecting the root element as focused
    let current_element = window()?
        .document()?
        .active_element()
        .and_then(|f| f.id().parse::<Uuid>().ok())
        .map(Id::from)
        .or(element_tree.elements.root)?;
    // log!(current_element.clone());
    log!(&event.text.as_str()); // TODo on hit enter this return code instead of table?
    let id = Id::new();
    let mut changes: Vec<EditorChange> = Vec::new();
    match event.text.as_str() {
        "table" => {
            // TODO we should hav ea generic way to create elements without using event.text.as_str()
            let (elements, adjacency) = get_example_table();
            for i in elements {
                let parent_id = adjacency
                    .iter()
                    .find(|(id, children)| {
                        for j in *children {
                            if *j == i.id {
                                return true;
                            }
                        }
                        return false;
                    })
                    .map(|(id, children)| id);
                changes.push(EditorChange::Create(EditorElementCreate {
                    id: i.id,
                    content: i.content,
                    attrs: i.attrs,
                    tag: i.tag,
                    tree_id: element_tree.id,
                    parent_id: *parent_id.unwrap_or(&current_element),
                    children: adjacency.get(&i.id).cloned(),
                    prev_element_id: None,
                }));
            }
            return Some((id, changes));
        }

        "code" => {
            changes.push(EditorChange::Create(EditorElementCreate {
                id,
                content: "E=mc^2".to_string(),
                tag: None,
                attrs: HashMap::from([(
                    "style".to_string(),
                    "padding:3ps; background: darkgreen; color: tomato;".to_string(),
                )]),
                tree_id: element_tree.id,
                parent_id: current_element,
                children: None,
                prev_element_id: None,
            }));
            return Some((id, changes));
        }
        "title" => {
            changes.push(EditorChange::Create(EditorElementCreate {
                id,
                content: "title is here.".to_string(),
                tag: Some("h1".to_string()),
                attrs: HashMap::new(),
                tree_id: element_tree.id,
                parent_id: current_element,
                children: None,
                prev_element_id: None,
            }));
            return Some((id, changes));
        }

        "image" => {
            changes.push(EditorChange::Create(EditorElementCreate {
                id,
                content: "".to_string(),
                tag: Some("img".to_string()),
                attrs: HashMap::from([
                    ("src".to_string(), "https://picsum.photos/200".to_string()),
                    ("style".to_string(), "float: left; width:100;".to_string()),
                ]),
                tree_id: element_tree.id,
                parent_id: current_element,
                children: None,
                prev_element_id: None,
            }));
            return Some((id, changes));
        }
        _ => {}
    }
    None
}

pub fn get_example_table() -> (Vec<EditorElement>, HashMap<Id, Vec<Id>>) {
    let root_table = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("table".to_string()),
    };
    let heading = EditorElement {
        id: Id::new(),
        content: "heading".to_string(),
        attrs: HashMap::new(),
        tag: Some("thead".to_string()),
    };
    let heading_row = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: None,
    };
    let year = EditorElement {
        id: Id::new(),
        content: "year".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
    };

    let company = EditorElement {
        id: Id::new(),
        content: "company".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
    };
    let table_body = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tbody".to_string()),
    };
    let table_row = EditorElement {
        id: Id::new(),
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
    };
    let table_row_el = EditorElement {
        id: Id::new(),
        content: "Alfreds Futterkiste".to_string(),
        attrs: HashMap::new(),
        tag: Some("td".to_string()),
    };
    let table_row_el2 = EditorElement {
        id: Id::new(),
        content: "2018".to_string(),
        attrs: HashMap::new(),
        tag: Some("td".to_string()),
    };
    let adjacency_list = HashMap::from([
        (root_table.id, vec![heading.id, table_body.id]),
        (heading.id, vec![company.id, year.id]),
        (table_body.id, vec![table_row.id]),
        (table_row.id, vec![table_row_el.id, table_row_el2.id]),
    ]);
    return (
        vec![
            root_table,
            heading,
            company,
            table_body,
            table_row,
            heading_row,
            table_row_el,
            table_row_el2,
            year,
        ],
        adjacency_list,
    );
}
