// editor/src/editor_
use std::collections::HashMap;
use js_sys::map;

use shared::id::Id;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{console, window, Element};
use yew::prelude::*;
use yew::virtual_dom::VTag;
use yew::{function_component, html};
use yewdux::log::kv::Source;

use shared::schema::EditorElement;

use crate::editor_components::{FromComponent, Table};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
    #[prop_or_default]
    pub children: Option<Vec<Id>>,
}

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
fn ConstructElement(props: &ConstractorProps) -> Html {
    let mut tag: Options<String> = props.tag.clone();
    let tag: String = format!("{}", tag.unwrap_or(String::from("div")));
    html! {
        <@{tag} id={{ props.id.to_string() }}>
            { &props.text }
            { props.children.clone() }
        </@>
    }
}

#[function_component]
pub fn EditorComponent(props: &Props) -> Html {
    let node: &EditorElement = &props.node;
    let tag: Options<String>  = node.tag.clone();
    let response: VNode = match tag.unwrap_or(String::from("")).as_str() {
        "table" => html! { 
            <Table> 
                { props.children.clone() }
            </Table>
        },
        "form" => html! { 
            <FromComponent /> 
        },
        _ => {
            let children: VNode = props.children.into_iter().map(|f: Vec<Id>| map.borrow().get(f).unwrap().to_owned()).collect::<Html>();
            html! {
                <ConstructElement
                    tag={ node.tag.clone() }
                    attrs={ node.clone().attrs }
                    id={ node.id }
                    text={ node.clone().text }
                >
                    { children }
                </ConstructElement>
            }
        }
    };

    html! {
        <>
            { response }
        </>
    }
}
