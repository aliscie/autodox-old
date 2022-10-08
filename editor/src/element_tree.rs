use serde::{Deserialize, Serialize};
use shared::Tree;
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};
use yew::prelude::*;
use yewdux::prelude::*;

pub type ElementId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default, Store)]
pub struct ElementTree {
    pub elements: Tree<ElementId, EditorElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct EditorElement {
    pub id: ElementId,
    pub text: String,
    pub attrs: HashMap<Attrs, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub enum Attrs {
    #[default]
    Style,
    Href,
    Src,
}

impl ElementTree {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            elements: Tree::new(),
        }
    }

    pub(crate) fn to_html(&self, start: u64) -> Html {
        let map: Rc<RefCell<HashMap<u64, Html>>> = Rc::new(RefCell::new(HashMap::new()));
        for (id, node) in self.elements.into_iter(start) {
            let mut has_children = false;
            if let Some(children) = self.elements.adjacency.get(id){
                has_children = children.len() > 0; 
            }
            let html_node = html! {
                <>
                <div style = { node.attrs
                    .get(&Attrs::Style).map(|e| e.clone())}
                    href = { node.attrs.get(&Attrs::Href).map(|e| e.clone())}
                    src = { node.attrs.get(&Attrs::Src).map(|e| e.clone())}
                >
                        { node.text.clone()}
                </div>
                <>
                    if has_children {{
                        self.elements.adjacency.get(id)
                        .unwrap()
                        .into_iter()
                        .map(|f| map.borrow().get(f).unwrap().to_owned())
                        .collect::<Html>()
                    }}
                </>
                </>
            };
            map.borrow_mut().insert(*id, html_node);
        }
        self.elements
            .adjacency
            .get(&start)
            .unwrap()
            .into_iter()
            .map(|f| map.borrow().get(f).unwrap().to_owned())
            .collect::<Html>()
    }
}

impl EditorElement {
    #[inline]
    pub fn new(id: u64, text: String, attrs: HashMap<Attrs, String>) -> Self {
        Self { id, text, attrs }
    }
}
