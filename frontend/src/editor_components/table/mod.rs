mod add_row;

// use crate::components::PopOverMenu;
use add_row::*;

mod add_column;

use add_column::*;

mod table_context_menu;
mod table_controls;
mod column_component;
mod cell_component;

use cell_component::Cell;
use column_component::Column;

use shared::schema::EditorElementCreate;
use std::collections::HashMap;
use table_context_menu::*;
use table_controls::TableControls;

use editor::plugins::Position;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, HtmlInputElement, HtmlTableCellElement, MouseEvent, HtmlElement};
use yew::prelude::*;
use yew::{function_component, html};
use yew_hooks::prelude::*;

use editor::GlobalEditorState;
use shared::*;

#[derive(Debug)]
pub struct TableIndex {
    row: usize,
    col: usize,
}

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

    let children = global_state
        .element_tree
        .elements
        .adjacency
        .get(&props.node.id)
        .unwrap();
    // shared::log!(&children);
    let thead = children
        .first()
        .and_then(|id| global_state.element_tree.elements.vertices.get(id))
        .and_then(|node| {
            let children = global_state
                .element_tree
                .elements
                .adjacency
                .get(&node.id)
                .unwrap_or(&Vec::new())
                .into_iter()
                .map(|heading| {
                    if let Some(el) = global_state.element_tree.elements.vertices.get(heading) {
                        let children = global_state
                            .element_tree
                            .elements
                            .adjacency
                            .get(&el.id)
                            .unwrap_or(&Vec::new())
                            .into_iter()
                            .map(|element| {
                                if let Some(el) =
                                    global_state.element_tree.elements.vertices.get(element)
                                {
                                    return html! { <td
                                    id = {el.id.to_string()}>{el.content.clone()}</td> };
                                }
                                html! {}
                            })
                            .collect::<Html>();
                        return html! {
                            <Column element={el.clone()}  >{children}</Column>
                        };
                    }
                    html! {}
                })
                .collect::<Html>();
            Some(html! {
                <thead id = {node.id.to_string()}>
                    {children}
                </thead>
            })
        });
    let mut row_number = 0;
    let mut col_number: isize = -1;
    let tbody = children
        .get(1)
        .and_then(|id| global_state.element_tree.elements.adjacency.get(id))
        .unwrap_or(&Vec::new())
        .into_iter()
        .map(|el| {
            if let Some(el) = global_state.element_tree.elements.vertices.get(el) {
                let children = global_state
                    .element_tree
                    .elements
                    .adjacency
                    .get(&el.id)
                    .unwrap_or(&Vec::new())
                    .into_iter()
                    .map(|table_cell| {
                        if let Some(table_cell) =
                            global_state.element_tree.elements.vertices.get(table_cell)
                        {
                            col_number += 1;
                            return html! {
                                <Cell node={props.node.clone()} element={table_cell.clone()} {row_number} col_number={col_number.clone() as usize}/>
                            };
                        }
                        html! {}
                    })
                    .collect::<Html>();
                row_number += 1;
                col_number = -1;
                return html! {
                    <tr id = { el.id.to_string()}>
                        {el.content.clone()}
                        {children}
                    </tr>
                };
            }
            html! {}
        })
        .collect::<Html>();

    html! {
    <>
    <div contenteditable="false" class={css_file_macro!("main.css")}>
        <TableControls />
    </div>
        <table style="width:100%" id = {props.node.id.to_string()}>
            {&props.node.content}
            {thead}
            {tbody}
        </table>
    </>
    }
}
