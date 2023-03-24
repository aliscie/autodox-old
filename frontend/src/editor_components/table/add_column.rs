use editor::GlobalEditorState;
use futures::TryFutureExt;
use shared::id::Id;
use shared::log;
use shared::schema::{EditorChange, EditorElementCreate};
use std::collections::HashMap;

pub fn add_col(global_state: &GlobalEditorState, table_id: &Id, col_number: usize) -> Option<()> {
    let root_table = global_state
        .element_tree
        .elements
        .adjacency
        .get(&table_id)?;

    // TODO
    //     based on the id insert the new column at to the right side of the clicked cell.

    // get the thead of table
    let thead = root_table
        .get(0)
        .and_then(|thead_id| global_state.element_tree.elements.adjacency.get(thead_id))?;
    let curr_column_id: Id = Id::try_from(*thead.get(col_number)?).ok()?;

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
        prev_element_id: Some(curr_column_id),
    }));
    for row_id in tbody_children {
        let current_cell_id = global_state
            .element_tree
            .elements
            .adjacency
            .get(row_id)?
            .get(col_number as usize)?;
        changes.push(EditorChange::Create(EditorElementCreate {
            id: Id::new(),
            content: "test".to_string(),
            attrs: HashMap::new(),
            tag: Some("td".to_string()),
            tree_id: global_state.element_tree.id,
            parent_id: *row_id,
            children: None,
            prev_element_id: Some(current_cell_id.clone()),
        }));
    }
    global_state.mutation.emit(changes);
    Some(())
}
