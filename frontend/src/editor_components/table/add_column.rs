use editor::GlobalEditorState;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElementCreate};
use std::collections::HashMap;

pub fn add_col(global_state: &GlobalEditorState, table_id: &Id) -> Option<()> {
    let root_table = global_state
        .element_tree
        .elements
        .adjacency
        .get(&table_id)?;

    // TODO Note: You don't need this
    //     let thead_row = root_table
    //         .first()
    //         .and_then(|thead_id| global_state.element_tree.elements.adjacency.get(thead_id)).unwrap().first().unwrap();
    //     .and_then(|thead_row| thead_row.first())?;

    let mut changes = Vec::new();

    let tbody_children = root_table
        .get(1)
        .and_then(|tbody_id| global_state.element_tree.elements.adjacency.get(tbody_id))?;
    changes.push(EditorChange::Create(EditorElementCreate {
        id: Id::new(),
        content: "test1".to_string(),
        attrs: HashMap::new(),
        tag: Some("th".to_string()),
        tree_id: global_state.element_tree.id,
        parent_id: root_table.first().unwrap().clone(),
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
