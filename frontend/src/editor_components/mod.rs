mod table;

// editor/src/editor_
use std::collections::HashMap;

use indexmap::IndexMap;
use shared::id::Id;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{console, window, Element};
use yew::prelude::*;
use yew::virtual_dom::{Attributes, VTag};
use yew::{function_component, html};
use shared::schema::EditorElement;
use table::Table;

use editor::editor_components::main::EditorElementProps;
// use crate::editor_components::table::GlobalEditorState;
// #[derive(Properties, PartialEq)]
// pub struct EditorElementPropsFE {
//     pub node: EditorElement,
//     #[prop_or_default]
//     pub children: Children,
// }

#[derive(Properties, PartialEq)]
pub struct ConstractorProps {
    tag: Option<String>,
    attrs: HashMap<String, String>,
    text: String,
    id: Id,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
fn ConstructElementFE(props: &ConstractorProps) -> Html {
    let mut tag = props.tag.clone();
    let tag = format!("{}", tag.unwrap_or(String::from("div")));
    let mut tag = VTag::new(tag);
    let mut attrs: IndexMap<AttrValue, AttrValue> = props
        .attrs
        .clone()
        .into_iter()
        .map(|(key, value)| -> (AttrValue, AttrValue) { (key.into(), value.into()) })
        .collect();
    attrs.insert("id".into(), props.id.to_string().into());
    tag.add_child(html! { {props.text.clone()}});
    tag.add_children(props.children.clone());
    tag.set_attributes(attrs);
    tag.into()
}

#[function_component]
pub fn EditorComponentFE(props: &EditorElementProps) -> Html {

    let node = &props.node;
    let tag = node.tag.clone();
    let response = match tag.unwrap_or(String::from("")).as_str() {
        "table" => html! { <Table node = {node.clone()}> {props.children.clone()}</Table>},
        "form" => html! { "<FromComponent/>"},
        _ => {
            html! {
            <ConstructElementFE
                tag={node.tag.clone()}
                attrs={node.clone().attrs}
                id = {node.id}
                text={node.clone().content}>
            {props.children.clone()}
            </ConstructElementFE>
            }
        }
    };

    html! {
        <>
        {response}
        </>

     }
}
