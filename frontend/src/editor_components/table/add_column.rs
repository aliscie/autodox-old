use editor::GlobalEditorState;
use futures::TryFutureExt;
use shared::id::Id;
use shared::log;
use shared::schema::{EditorChange, EditorElementCreate};
use std::collections::HashMap;

pub fn add_col(index: i32, global_state: &GlobalEditorState, table_id: &Id) -> Option<()> {
    let root_table_children = global_state
        .element_tree
        .elements
        .adjacency
        .get(&table_id)?;
    let thead_row = root_table_children
        .first()
        .and_then(|thead_id| global_state.element_tree.elements.adjacency.get(thead_id))
        .and_then(|thead_row| thead_row.first())?;
    let mut changes = Vec::new();
    let tbody_children = root_table_children
        .get(1)
        .and_then(|tbody_id| global_state.element_tree.elements.adjacency.get(tbody_id))?;
    changes.push(EditorChange::Create(EditorElementCreate {
        id: Id::new(),
        content: "test".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
        tree_id: global_state.element_tree.id,
        parent_id: *thead_row,
        children: None,
        prev_element_id: None,
    }));
    for row_id in tbody_children {
        changes.push(EditorChange::Create(EditorElementCreate {
            id: Id::new(),
            content: "test".to_string(),
            attrs: HashMap::new(),
            tag: Some("td".to_string()),
            tree_id: global_state.element_tree.id,
            parent_id: *row_id,
            children: None,
            prev_element_id: None,
        }));
    }
    global_state.mutation.emit(changes);
    Some(())
}
