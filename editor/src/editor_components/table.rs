use crate::plugins::Position;
use shared::id::Id;
use shared::log;
use shared::schema::EditorElement;
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
    let table_id = props.table_id.clone();
    let add_row = {
        let global_state = global_state.clone();
        use_callback(
            move |e: MouseEvent, _| {
                let number_of_col = global_state
                    .element_tree
                    .elements
                    .adjacency
                    .get(&table_id)
                    .and_then(|root_table_children| {
                        // get the first children as that is the thead element!
                        root_table_children.first()
                    })
                    .and_then(|thead_id| {
                        // getting the thead children
                        global_state.element_tree.elements.adjacency.get(thead_id)
                    })
                    .map(|thead_children| {
                        // the number of children thead has should be equal to
                        // the number of columns in the table!
                        thead_children.len()
                    });
            },
            (),
        )
    };
    html! {
        <>
        <button>
        {"Add row!"}
        </button>
        <button>
        {"Add Column!"}
        </button>
        </>
    }
}
