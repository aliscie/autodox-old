use editor::GlobalEditorState;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElementCreate};
use std::collections::HashMap;
use shared::log;

pub fn add_row(global_state: &GlobalEditorState, table_id: &Id) -> Option<()> {

    let root_table_children = global_state
        .element_tree
        .elements
        .adjacency
        .get(&table_id)?;

    let number_of_col = global_state
        .element_tree
        .elements
        .adjacency
        .get(root_table_children.first()?)?
        .first()
        .and_then(|thead_id| global_state.element_tree.elements.adjacency.get(thead_id))?
        .len();
    log!("add row");
    let tbody_id = root_table_children.get(1)?;
    let prev_element_id = global_state
        .element_tree
        .elements
        .adjacency
        .get(tbody_id)
        .and_then(|row| row.last());
    let row_id = Id::new();
log!("add row");
    let mut changes = vec![EditorChange::Create(EditorElementCreate {
        id: row_id,
        content: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
        tree_id: global_state.element_tree.id,
        parent_id: *tbody_id,
        children: None,
        prev_element_id: prev_element_id.cloned(),
    })];
log!("add row");
    for i in 0..number_of_col {
        changes.push(EditorChange::Create(EditorElementCreate {
            id: Id::new(),
            content: "test".to_string(),
            attrs: HashMap::new(),
            tag: Some("td".to_string()),
            tree_id: global_state.element_tree.id,
            parent_id: row_id,
            children: None,
            prev_element_id: None,
        }))
    }
    log!("add row");
    global_state.mutation.emit(changes);
    log!("add row 2");
    Some(())
}
