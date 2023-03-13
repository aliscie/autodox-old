use crate::EditorElementProps;
use shared::id::Id;
use shared::schema::ElementTree;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

pub(crate) fn render<T>(tree: &ElementTree, start: Id) -> Html
where
    T: BaseComponent + BaseComponent<Properties = EditorElementProps>,
{
    let map: Rc<RefCell<HashMap<Id, Html>>> = Rc::new(RefCell::new(HashMap::new()));
    for (id, node) in tree.elements.into_iter(start) {
        let mut has_children = false;
        if let Some(children) = tree.elements.adjacency.get(id) {
            has_children = !children.is_empty();
        }

        let html_node = html! {
            <T
                key = { id.to_string() }
                node={node.clone()}>
                if has_children {{
                    tree.elements.adjacency.get(id)
                        .unwrap()
                        .into_iter()
                        .map(|f| map.borrow().get(f).unwrap().to_owned())
                        .collect::<Html>()
                }}
            </T>

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
