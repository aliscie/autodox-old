// editor/src/editor_
use std::collections::HashMap;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, Element};
use yew::prelude::*;
use yew::{function_component, html};

use shared::schema::EditorElement;

use crate::editor_components::{
    Calendar,
    Form, 
    Image,
    Qout,
    Table,
    Video
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[derive(Properties, PartialEq)]
pub struct ConstractorProps {
    tag: Option<String>,
    attrs: HashMap<String, String>,
    text: String,
}

#[function_component]
fn ConstructElement(props: &ConstractorProps) -> Html {
    let mut tag = props.tag.clone();
    if tag.is_none() {
        tag = Some("p".to_string());
    }
    let doc = window().unwrap_throw().document().unwrap_throw();
    let new_element: Element = doc.create_element(&tag.unwrap()).unwrap();
    new_element.set_inner_html(&props.text);

    for (key, value) in props.attrs.iter() {
        new_element.set_attribute(key, value).unwrap();
    }
    Html::VRef(new_element.unchecked_into())
}

#[function_component]
pub fn EditorComponent(props: &Props) -> Html {
    let node = &props.node;
    if node.tag.is_none() {
        return html! { <p>{ &node.text }</p>} ;
    }
    let tag = node.tag.clone();    
    let response = match tag.clone().unwrap().as_str() {
        "calendar" => html! { <Calendar /> },
        "form" => html! { <Form /> },
        "image" => html! { <Image />} ,
        "qout" => html! { <Qout /> },
        "table" => html! { <Table /> },
        "video" => html! { <Video /> },
        _ => {
            html! {
                <ConstructElement 
                    tag={ tag } 
                    attrs={ node.clone().attrs } 
                    text={ node.clone().text }
                />
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
            { response }
        </>
    //</span>
     }
}
