mod add_row;

// use crate::components::PopOverMenu;
use add_row::*;

mod add_column;

use add_column::*;

mod table_context_menu;
mod table_controls;
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
    let on_column_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let curr :HtmlElement = mouse_event.target_unchecked_into();

            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![
                    html!{<a
                        onclick={
                            Callback::from(move |e: MouseEvent| {
                                curr.set_attribute("formula", "1+1").unwrap_throw();
                            })
                        }
                        >{"add formula"}</a>},
                    html!{<a>{"add filter"}</a>},]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state
                .render_context_menu
                .emit((mouse_event, element))
        })
    };

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
                            <th
                            oncontextmenu={on_column_contextmenu.clone()}
                                id = {el.id.to_string()}>
                                {el.content.clone()}
                                {children}
                            </th>
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
    let add_row_callback = |row_number: usize, global_state: GlobalEditorState, table_id: Id| {
        Callback::from(move |e: MouseEvent| {
            let _ = add_row::add_row(&global_state, &table_id, row_number);
        })
    };
    let add_col_callback = |col: usize, global_state: GlobalEditorState, table_id: Id| {
        Callback::from(move |e: MouseEvent| {
            log!(col);
            let _ = add_column::add_col(&global_state, &table_id, col);
        })
    };
    let oncontextmenu = {
        let table_id = table_id.clone();
        move |row: usize, col: usize, global_state: GlobalEditorState| {
            Callback::from(move |mouse_event: MouseEvent| {
                let element = html! {
                    <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                        <a onclick={add_row_callback(row, global_state.clone(), table_id.clone())}>{"add row"}</a>
                        <a onclick={add_col_callback(col, global_state.clone(), table_id.clone())}>{"add column"}</a>
                    </ContextProvider<GlobalEditorState>>
                };
                global_state
                    .render_context_menu
                    .emit((mouse_event, element))
            })
        }
    };
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
                                <td
                                oncontextmenu={oncontextmenu(row_number, col_number as usize, global_state.clone())}
                                id = { table_cell.id.to_string()}> {table_cell.content.clone()}</td>
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
