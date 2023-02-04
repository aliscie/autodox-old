use crate::EditorChange;
use shared::id::Id;
use shared::log;
use shared::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, MutationRecord};
use yew::Callback;

pub fn handle_mutation(
    mutation: MutationRecord,
    onchange: &Callback<EditorChange>,
    element_tree: Rc<RefCell<ElementTree>>,
) {
    if let Some(current_element) = mutation.target() {
        match mutation.type_().as_ref() {
            "characterData" => {
                if let Some(parent_element) = current_element.parent_element() {
                    if let Ok(id) = Uuid::parse_str(parent_element.id().as_ref()).map(Id::from) {
                        //log!(&format!("{:?}", parent_element.inner_html()).into());
                        //log!(&format!("{:?}", id).into());
                        let update = EditorElementUpdate {
                            id,
                            text: Some(parent_element.inner_html().clone()),
                            ..Default::default()
                        };
                        onchange.emit(EditorChange::Update(update));
                    }
                }
            }
            "attributes" => {
                if let Some(parent_element) = current_element.parent_element() {
                    log!(format!("Got create: {:?}", parent_element.inner_html()));
                }
            }
            "childList" => {
                let removed_nodes = mutation.removed_nodes();
                for i in 0..removed_nodes.length() {
                    removed_nodes
                        .get(i)
                        .and_then(|node| node.dyn_into::<Element>().ok())
                        .and_then(|element| Uuid::parse_str(element.id().as_str()).ok())
                        .map(Id::from)
                        .and_then(|id| {
                            let parent_id = element_tree
                                .as_ref()
                                .borrow()
                                .elements
                                .adjacency
                                .iter()
                                .find(|(_, vec)| vec.contains(&id))?
                                .0
                                .clone();
                            let delete = EditorElementDelete {
                                id,
                                tree_id: element_tree.as_ref().borrow().id.clone(),
                                parent_id,
                            };
                            onchange.emit(EditorChange::Delete(delete));
                            Some(())
                        });
                }
                if removed_nodes.length() > 0 {
                    // move to next mutation record!
                    // crate::shared::log!("got element delete!");
                    // crate::shared::log!(mutation.removed_nodes());
                    return;
                }
                let element = current_element.unchecked_into::<Element>();
                if element.id() == "text_editor" {
                    return;
                }
                let new_id = Uuid::new_v4();
                let mut prev_element_id: Option<Id> = None;
                if let Some(prev_node) = element.previous_sibling() {
                    let prev_element = prev_node.unchecked_into::<Element>();
                    // crate::shared::log!(format!("previous element id : {:?}", prev_element.id()));
                    prev_element_id = Uuid::parse_str(prev_element.id().as_str())
                        .map(Id::from)
                        .ok();
                }
                let element_create = EditorElementCreate {
                    id: new_id.into(),
                    text: element.text_content().unwrap_or_default(),
                    attrs: HashMap::new(),
                    tree_id: element_tree.as_ref().borrow().id,
                    parent_id: element_tree.as_ref().borrow().elements.root.unwrap(),
                    prev_element_id,
                    children: None,
                };
                onchange.emit(EditorChange::Create(element_create));
                element.set_id(&new_id.to_string());
            }
            anything_else => unimplemented!(), //crate::shared::log!(anything_else),
        }
    }
}
