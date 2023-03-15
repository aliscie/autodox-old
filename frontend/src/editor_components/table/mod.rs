mod add_row;

use add_row::*;

mod add_column;

use add_column::*;

mod table_context_menu;

use table_context_menu::*;
use shared::schema::EditorElementCreate;
use std::collections::HashMap;

use editor::plugins::Position;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, MouseEvent, window};
use yew::prelude::*;
use yew::{function_component, html};
use yew_hooks::prelude::*;

use editor::GlobalEditorState;
use shared::log;

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
    let filters_items: Vec<Html> = vec![
        html! {<option>{"my filter"}</option>},
        html! {<option>{"other filer"}</option>},
        html! {<option>{"todos filer"}</option>},
        html! {<option>{"Add a filter +"}</option>},
    ];
    let views: Vec<Html> = vec![
        html! {<option>{"grid"}</option>},
        html! {<option>{"other"}</option>},
        html! {<option>{"blab_blab"}</option>},
        html! {<option>{"Add a view +"}</option>},
    ];
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");
    let table_id = props.node.id.clone();

    let add_row_callback = {
        let global_state = global_state.clone();
        let table_id = table_id.clone();
        Callback::from(move |e: MouseEvent| {
            let _ = add_row::add_row(&global_state, &table_id);
        })
    };


    let oncontextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let curr: HtmlInputElement = mouse_event.target_unchecked_into();
            let id = curr.id();
            let id: Id = Id::try_from(id).unwrap_throw();

            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![html!{<a onclick={add_row_callback.clone()}>{"add row"}</a>}, html!{<a
                    onclick={
                        let global_state = global_state.clone();
                        let table_id = table_id.clone();
                        Callback::from(move |e: MouseEvent| {
                            let _ = add_column::add_col(id, &global_state, &table_id);
                        })
                    }

                    >{"add column"}</a>}]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state.render_context_menu.emit((mouse_event, element))
        })
    };
    let children = global_state
        .element_tree
        .elements
        .adjacency
        .get(&props.node.id)
        .unwrap();
    shared::log!(&children);
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
                            <th id = {el.id.to_string()}>
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
                        if let Some(table_cell) = global_state.element_tree.elements.vertices.get(table_cell) {
                            return html! {
                                <td
                                oncontextmenu={oncontextmenu.clone()}
                                id = { table_cell.id.to_string()}> {table_cell.content.clone()}</td>
                            };
                        }
                        html! {}
                    }).collect::<Html>();
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
        <table id = {props.node.id.to_string()}>
            {&props.node.content}
            {thead}
            {tbody}
        </table>
    </>
    }
}
