use shared::schema::EditorElementCreate;
use std::collections::HashMap;

use crate::plugins::Position;
use shared::id::Id;
use shared::log;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use yew::{function_component, html};
use yew_hooks::prelude::*;

use crate::app::GlobalEditorState;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
    pub node: EditorElement,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Table(props: &Props) -> Html {
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");
    let table_id = props.node.id.clone();
    let oncontextmenu = Callback::from(move |e: MouseEvent| {
        let element = html! {
            <TableContextMenu event = {e.clone()} {table_id}/>
        };
        global_state.render_context_menu.emit((e, element))
    });

    html! {
    <>
        <table id = {props.node.id.to_string()} {oncontextmenu}>
            {&props.node.text}
            {props.children.clone()}
        </table>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableContextMenuProps {
    event: MouseEvent,
    table_id: Id,
}

#[function_component]
pub fn TableContextMenu(props: &TableContextMenuProps) -> Html {
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");
    let add_row_callback = {
        let global_state = global_state.clone();
        let table_id = props.table_id.clone();
        Callback::from(move |e: MouseEvent| {
            let _ = add_row(&global_state, &table_id);
        })
    };
    let add_col_callback = {
        let global_state = global_state.clone();
        let table_id = props.table_id.clone();
        Callback::from(move |e: MouseEvent| {
            let _ = add_col(&global_state, &table_id);
        })
    };
    html! {
        <>
        <button onclick = {add_row_callback}>
        {"Add row!"}
        </button>
        <button onclick = {add_col_callback}>
        {"Add Column!"}
        </button>
        </>
    }
}

fn add_col(global_state: &GlobalEditorState, table_id: &Id) -> Option<()> {
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
        text: "test".to_string(),
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
            text: "test".to_string(),
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

fn add_row(global_state: &GlobalEditorState, table_id: &Id) -> Option<()> {
    let root_table_children = global_state
        .element_tree
        .elements
        .adjacency
        .get(&table_id)?;
    let number_of_col = root_table_children
        .first()
        .and_then(|thead_id| {
            // getting the thead children
            global_state.element_tree.elements.adjacency.get(thead_id)
        })
        .map(|thead_children| {
            // the number of children thead has should be equal to
            // the number of columns in the table!
            thead_children.len()
        })?;
    let tbody_id = root_table_children.get(1)?;
    let prev_element_id = global_state
        .element_tree
        .elements
        .adjacency
        .get(tbody_id)
        .and_then(|row| row.last());
    let row_id = Id::new();
    let mut changes = vec![EditorChange::Create(EditorElementCreate {
        id: row_id,
        text: "".to_string(),
        attrs: HashMap::new(),
        tag: Some("tr".to_string()),
        tree_id: global_state.element_tree.id,
        parent_id: *tbody_id,
        children: None,
        prev_element_id: prev_element_id.cloned(),
    })];
    for i in 0..number_of_col {
        changes.push(EditorChange::Create(EditorElementCreate {
            id: Id::new(),
            text: "test".to_string(),
            attrs: HashMap::new(),
            tag: Some("td".to_string()),
            tree_id: global_state.element_tree.id,
            parent_id: row_id,
            children: None,
            prev_element_id: None,
        }))
    }
    global_state.mutation.emit(changes);
    Some(())
}
