use crate::plugins::Position;
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

    html! {
    <>
        <table id = {props.node.id.to_string()} >
            {&props.node.text}
            {props.children.clone()}
        </table>
    </>
    }
}
