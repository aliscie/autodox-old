use crate::editor_components::EditorComponent;
use shared::id::Id;
use shared::schema::ElementTree;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

pub(crate) fn render(tree: &ElementTree, start: Id) -> Html {
    let map: Rc<RefCell<HashMap<Id, Html>>> = Rc::new(RefCell::new(HashMap::new()));
    for (id, node) in tree.elements.into_iter(start) {
        let mut has_children = false;
        if let Some(children) = tree.elements.adjacency.get(id) {
            has_children = !children.is_empty();
        }

        let html_node = html! {
            <>
                <EditorComponent
                key = { id.to_string() }
                node={node.clone()}/>
                if has_children {{
                    tree.elements.adjacency.get(id)
                        .unwrap()
                        .into_iter()
                        .map(|f| map.borrow().get(f).unwrap().to_owned())
                        .collect::<Html>()
                }}
            </>
        };
        map.borrow_mut().insert(*id, html_node);
    }
    tree.elements
        .adjacency
        .get(&start)
        .unwrap()
        .into_iter()
        .map(|f| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
}
