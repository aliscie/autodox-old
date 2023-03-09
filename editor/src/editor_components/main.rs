// editor/src/editor_
use std::collections::HashMap;

use shared::id::Id;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{console, window, Element};
use yew::prelude::*;
use yew::virtual_dom::VTag;
use yew::{function_component, html};

use shared::schema::EditorElement;

use crate::editor_components::{FromComponent, Table};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
    #[prop_or_default]
    pub children: Children,
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
    let mut tag = props.tag.clone();
    let tag = format!("{}", tag.unwrap_or(String::from("div")));
    html! {
        <@{tag} id = {{props.id.to_string()}}>
            {&props.text}
            { props.children.clone() }
        </@>
    }
}

#[function_component]
pub fn EditorComponent(props: &Props) -> Html {
    let node = &props.node;
    let tag = node.tag.clone();
    let response = match tag.unwrap_or(String::from("")).as_str() {
        "table" => html! { <Table> {props.children.clone()}</Table>},
        "form" => html! { <FromComponent/>},
        _ => {
            html! {
            <ConstructElement
                tag={node.tag.clone()}
                attrs={node.clone().attrs}
                id = {node.id}
                text={node.clone().content}>
            {props.children.clone()}
            </ConstructElement>
            }
        }
    };

    // don't use this use a better method
    //let doc = window().unwrap_throw().document().unwrap_throw();

    //if let Some(node) = node_ref.clone().cast::<Element>() {
    //// add event lsitner to node_ref cast :: element
    //let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
    //log!("onchage");
    //}) as Box<dyn FnMut(MouseEvent)>);
    //node.add_event_listener_with_callback("onchange", closure.as_ref().unchecked_ref()).unwrap_throw();
    //closure.forget();
    //}

    // let _position = position.clone();
    // let _node_ref = node_ref.clone();
    // let handle_hovering = Closure::wrap(Box::new(move |_e: MouseEvent| {
    //     let element = _node_ref.clone().cast::<Element>();
    //     if element.is_some() {
    //         let rec = element.unwrap().get_bounding_client_rect();
    //         let top = rec.top();
    //         let left = rec.left();
    //         let bottom = rec.bottom();
    //         let y = _e.client_y() as f64;
    //         if y>= top && y <= bottom {
    //             _position.set(format!(
    //                 "display:inline-block; left:{}px",
    //                 left - (25 as f64)
    //             ))
    //         } else {
    //             _position.set("display:none".to_string())
    //         }
    //     }
    // }) as Box<dyn FnMut(_)>);
    // let _ = &doc
    //     .get_element_by_id("text_editor_container")
    //     .unwrap()
    //     .add_event_listener_with_callback("mousemove", &handle_hovering.as_ref().unchecked_ref());
    // handle_hovering.forget();

    html! {
    //<span ref={node_ref} >
             //<Drag position={format!("{}",*(position.clone()))}/>
        <>
        {response}
        </>
    //</span>
     }
}
