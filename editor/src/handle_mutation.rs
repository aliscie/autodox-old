use shared::id::Id;
use shared::log;
use shared::schema::EditorChange;
use shared::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, MutationRecord};
use yew::prelude::*;
use yew::Callback;

pub fn handle_mutation(
    mutation_event: &Vec<MutationRecord>,
    onchange: &Callback<EditorChange>,
    element_tree: &mut ElementTree,
) -> Option<()> {
    for mutation in mutation_event {
        if let Some(current_element) = mutation.target() {
            match mutation.type_().as_ref() {
                "characterData" => {
                    if let Some(parent_element) = current_element.parent_element() {
                        if let Ok(id) = Uuid::parse_str(parent_element.id().as_ref()).map(Id::from)
                        {
                            let update_data = EditorChange::Update(EditorElementUpdate {
                                id,
                                content: parent_element.text_content(),
                                tree_id: element_tree.id.clone(),
                                attrs: None,
                                parent: None,
                                children: None,
                            });
                            element_tree.mutate_tree(&update_data);
                            onchange.emit(update_data);
                        }
                    }
                }
                "attributes" => {
                    if let Some(element) = current_element.parent_element() {
                        if let Ok(id) = Uuid::parse_str(element.id().as_ref()).map(Id::from) {
                            log!("get element attrs here and fire element_update");
                        }
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
                                    .elements
                                    .adjacency
                                    .iter()
                                    .find(|(_, vec)| vec.contains(&id))?
                                    .0
                                    .clone();
                                let delete = EditorElementDelete {
                                    id,
                                    tree_id: element_tree.id.clone(),
                                    parent_id,
                                };
                                let delete = EditorChange::Delete(delete);
                                element_tree.mutate_tree(&delete);
                                onchange.emit(delete);
                                Some(())
                            });
                    }
                    let element = current_element.unchecked_into::<Element>();
                    if element.id() == "text_editor" {
                        continue;
                    }
                    if let Ok(id) = element.id().parse::<Uuid>().map(Id::from) {
                        if element_tree.elements.vertices.contains_key(&id) {
                            continue;
                        }
                    }
                    let mut changes = Vec::new();
                    let parent_id = element
                        .parent_element()
                        .map(|el| el.id())
                        .and_then(|id| Uuid::parse_str(&id).ok())
                        .map(Id::from)
                        .or(element_tree.elements.root)
                        .unwrap();
                    create_element(&mut changes, element, parent_id, element_tree.id.clone());
                    let added_nodes = mutation.added_nodes();
                    for i in 0..added_nodes.length() {
                        let node = added_nodes.get(i).unwrap();
                        if node.node_name() == "#text" {
                            continue;
                        }
                        let element = node.dyn_into::<Element>().unwrap();
                        let parent_id = element
                            .parent_element()
                            .map(|el| el.id())
                            .and_then(|id| Uuid::parse_str(&id).ok())
                            .map(Id::from)
                            .or(element_tree.elements.root.clone())
                            .unwrap();
                        create_element(&mut changes, element, parent_id, element_tree.id.clone());
                    }
                    for i in changes {
                        element_tree.mutate_tree(&i);
                        onchange.emit(i);
                    }
                }
                anything_else => unimplemented!(), //crate::shared::log!(anything_else),
            }
        }
    }
    // log!(element_tree.;
    Some(())
}

fn create_element(changes: &mut Vec<EditorChange>, element: Element, parent_id: Id, tree_id: Id) {
    let new_id = Id::new();
    log!(element.id());
    let prev_element_id = element
        .previous_element_sibling()
        .map(|el| el.id())
        .and_then(|id| Uuid::parse_str(&id).ok())
        .map(Id::from);
    log!(&prev_element_id);
    let create = EditorElementCreate {
        id: new_id,
        content: element.text_content().unwrap_or_default(),
        tag: Some(element.tag_name()),
        attrs: HashMap::new(),
        tree_id,
        parent_id,
        prev_element_id,
        children: None,
    };
    element.set_id(&new_id.to_string());
    changes.push(EditorChange::Create(create));
}
