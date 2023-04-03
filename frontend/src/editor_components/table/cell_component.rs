use web_sys::{HtmlElement, MouseEvent};
use yew::{Callback, Children, html, Html};
use yew::prelude::*;
use editor::GlobalEditorState;
use shared::schema::EditorElement;
use super::table_context_menu::*;
use wasm_bindgen::UnwrapThrowExt;
use shared::id::Id;
use crate::editor_components::table::{add_column, add_row};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
    pub node: EditorElement,
    pub row_number: usize,
    pub col_number: usize,
    pub element: EditorElement,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Cell(props: &Props) -> Html {
    let table_id = props.node.id.clone();
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");
    let add_row_callback = |row_number: usize, global_state: GlobalEditorState, table_id: Id| {
        Callback::from(move |e: MouseEvent| {
            let _ = add_row::add_row(&global_state, &table_id, row_number);
        })
    };
    let add_col_callback = |col: usize, global_state: GlobalEditorState, table_id: Id| {
        Callback::from(move |e: MouseEvent| {
            // log!(col);
            let _ = add_column::add_col(&global_state, &table_id, col);
        })
    };
    let on_column_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let curr: HtmlElement = mouse_event.target_unchecked_into();

            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![
                    html!{<><a
                        onclick={
                            Callback::from(move |e: MouseEvent| {
                                curr.set_attribute("formula", "1+1").unwrap_throw();
                            })
                        }
                        >{"add formula"}</a>
                        <a>{"add filter"}</a>
                        <a>{"somthing"}</a>
                    </>},
                    ]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state
                .render_context_menu
                .emit((mouse_event, element))
        })
    };
    let id = props.element.id.clone();
    let content = props.element.content.clone();
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
    html! {
        <td
            oncontextmenu={oncontextmenu(props.row_number.clone(), props.col_number.clone(), global_state.clone())}
            id = { id.to_string()}> {content.clone()}</td>
    }
}