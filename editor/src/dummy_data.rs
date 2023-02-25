use shared::id::Id;
use shared::schema::{EditorElement, ElementTree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub fn generate_dummy() -> ElementTree {
    let mut default_element_tree = ElementTree::default();
    let root = default_element_tree.elements.root.unwrap();
    let id: Id = Uuid::new_v4().into();
    let new_element = EditorElement {
        id: Uuid::new_v4().into(),
        text: "this is a bold and italic".to_string(),
        tag: Some("i".to_string()),
        attrs: HashMap::new(),
    };
    default_element_tree.elements.push_children(
        root,
        id.clone(),
        EditorElement {
            id,
            text: "this is a bold text".to_string(),
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

    default_element_tree
}
