use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use shared::{
    id::Id,
    log,
    schema::{EditorElement, ElementTree},
};
use uuid::Uuid;
use web_sys::{window, Range};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{plugins::DropDownItem, render::render};

pub fn on_slash_input(
    event: DropDownItem,
    range: Option<Range>,
    element_tree: &mut ElementTree,
) -> Option<()> {
    // if no focused element is found we are selecting the root element as focused
    let current_element = window()?
        .document()?
        .active_element()
        .and_then(|f| f.id().parse::<Uuid>().ok())
        .map(Id::from)
        .or(element_tree.elements.root)?;
    // log!(current_element.clone());
    log!(&event.text.as_str()); // TODo on hit enter this return code instead of table?
    match event.text.as_str() {
        "table" => {
            // TODO we should hav ea generic way to create elements without using event.text.as_str()
            let (elements, adjacency) = get_example_table();
            {
                element_tree
                    .elements
                    .adjacency
                    .get_mut(&current_element)
                    .map(|f| {
                        f.push(elements[0].id);
                    });
                for i in elements {
                    element_tree.elements.push_vertex(i.id, i);
                }
                for (id, children_id) in adjacency {
                    element_tree.elements.adjacency.insert(id, children_id);
                }
            }
        }
        _ => {}
    }
    Some(())
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
        content: "heading row".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
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
    let adjacency_list = HashMap::from([
        (root_table.id, vec![heading.id, table_body.id]),
        (heading.id, vec![heading_row.id]),
        (heading_row.id, vec![company.id]),
        (table_body.id, vec![table_row.id]),
        (table_row.id, vec![table_row_el.id]),
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
        ],
        adjacency_list,
    );
}
