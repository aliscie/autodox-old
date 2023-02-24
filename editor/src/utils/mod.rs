use std::{cell::RefCell, collections::HashMap, rc::Rc};

use shared::{
    id::Id,
    log,
    schema::{EditorElement, ElementTree},
};
use uuid::Uuid;
use web_sys::{window, Range};
use yewdux::prelude::Dispatch;

use crate::{plugins::DropDownItem, render::render};

pub fn on_slash_input(
    event: DropDownItem,
    range: Option<Range>,
    element_tree: Rc<RefCell<ElementTree>>,
) -> Option<()> {
    // if no focused element is found we are selecting the root element as focused
    let current_element = window()?
        .document()?
        .active_element()
        .and_then(|f| f.id().parse::<Uuid>().ok())
        .map(Id::from)
        .or(element_tree.as_ref().borrow().elements.root)?;
    // log!(current_element.clone());
    log!(&event.text.as_str()); // TODo on hit enter this return code instead of table?
    match event.text.as_str() {
        "table" => {
            // TODO we should hav ea generic way to create elements without using event.text.as_str()
            let (elements, adjacency) = get_example_table();
            {
                let mut value = element_tree.as_ref().borrow_mut();
                value.elements.adjacency.get_mut(&current_element).map(|f| {
                    f.push(elements[0].id);
                });
                for i in elements {
                    value.elements.push_vertex(i.id, i);
                }
                for (id, children_id) in adjacency {
                    value.elements.adjacency.insert(id, children_id);
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
        text: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("table".to_string()),
    };
    let heading = EditorElement {
        id: Id::new(),
        text: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
    };
    let company = EditorElement {
        id: Id::new(),
        text: "Company".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
    };
    let table_body = EditorElement {
        id: Id::new(),
        text: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
    };
    let table_row = EditorElement {
        id: Id::new(),
        text: "Alfreds Futterkiste".to_string(),
        attrs: HashMap::new(),
        tag: Some("td".to_string()),
    };
    let adjacency_list = HashMap::from([
        (root_table.id, vec![heading.id, table_body.id, table_body.id]),
        (heading.id, vec![company.id, company.id, company.id]),
        (table_body.id, vec![table_row.id, table_row.id, table_row.id]),
    ]);
    (vec![root_table, heading, company, table_body, table_row], adjacency_list)
}
