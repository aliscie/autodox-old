use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::{function_component, html};
use yew::prelude::*;
use web_sys::{Element, MouseEvent, window};
use shared::schema::EditorElement;
use crate::editor_components::{Table, FromComponent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[function_component]
pub fn EditorComponent(props: &Props) -> Html {
    //let node_ref = NodeRef::default();
    //let position: UseStateHandle<String> = use_state(|| "display:none".to_string());
    let node = &props.node;
    let doc = window().unwrap_throw().document().unwrap_throw();
    if node.tag.is_none() {
        return html! {<p>{&node.text}</p>};
    }

    let response = match node.clone().tag.unwrap().as_str() {
        "table" => html! { <Table/>},
        "form" => html! { <FromComponent/>},
        _tag => {
            // let mut element = doc.create_html_element("p");
            // element.set_attribute("id", &node.id.to_string());
            // if _tag != "None" {
            //     element = doc.create_html_element(_tag);
            //     for key, value in node.clone().attrs
            //     {
            //         element.set_attribute(key, valure);
            //     };
            // };
            // html! {element}
            html! {<p
                // id={node.id}
                >{node.clone().text}</p>}
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
    //         if y >= top && y <= bottom {
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
    // let mut element = doc.create_element(node.tag.unwrap())
    // element.innert_text(node.text);
    // for at in nmod.attr {
    //     element.add_attributes(at.name, at.value);
    // }

    html! {
    //<span ref={node_ref}  >
             //<Drag position={format!("{}",*(position.clone()))}/>
        <>
        {response}
        </>
    //</span>
     }
}
