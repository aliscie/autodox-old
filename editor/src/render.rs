use crate::editor_components::EditorComponent;
use shared::id::Id;
use shared::schema::{EditorElement, ElementTree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew::{html};

pub(crate) fn render(tree: &ElementTree, start: Id) -> Html {
    let map: Rc<RefCell<HashMap<Id, Html>>> = Rc::new(RefCell::new(HashMap::new()));

    for (id, node) in tree.elements.into_iter(start) {
        let mut has_children: bool = false;
        if let Some(children) = tree.elements.adjacency.get(id) {
            has_children = !children.is_empty();
        }

        let children: Option<Vec<Id>> = Some(tree.elements.adjacency.get(id).unwrap().clone());

        let html_node: VNode = html! {
            <EditorComponent 
                key={ id.to_string() } 
                node={ node.clone() } 
            >
                { children }
            </EditorComponent>
        };

        map.borrow_mut().insert(*id, html_node);
    }

    tree.elements
        .adjacency
        .get(&start)
        .unwrap()
        .into_iter()
        .map(|f: &Id| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
}
