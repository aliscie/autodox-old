use yew::prelude::*;
use yew::{function_component, html, Html, UseStateHandle};
use crate::element_tree::EditorElement;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, MouseEvent, window};
use wasm_bindgen::{UnwrapThrowExt, JsCast};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[function_component(Render)]
pub fn render(props: &Props) -> Html {
    let node_ref = NodeRef::default();
    let position: UseStateHandle<&str> = use_state(|| "none");
    let node = props.node.clone();

    let doc = window().unwrap_throw().document().unwrap_throw();
    let _position = position.clone();
    let handle_hovering = Closure::wrap(Box::new(move |_e: MouseEvent| {
        // let mut element = _node_ref.clone().cast::<Element>();
        // let top = element.unwrap().unwrap().rec().top();
        // let buttom = element.unwrap().unwrap().rec().top();
        // if top >= _e.page_y()  && _e.page_y() >= buttom  && *_position == ""{
        //    _position.set("inline-block")
        // }  else {
        //     _position.set("")
        // }
        //
    }) as Box<dyn FnMut(_)>);
    let _ = &doc.add_event_listener_with_callback("mousemove", &handle_hovering.as_ref().unchecked_ref());


    html! {
    <span ref={node_ref} >
        <div style={format!("transition: 0.2s; position: absolute; left:50px; display: {};",*(position.clone()))}>
            <i contenteditable="false" id="drag-icon" style="display: inline-block;">{"+"}</i>
            <i contenteditable="false" id="drag-icon" style="display: inline-block;">{"::"}</i>
        </div>
        <div
                    // style = { &node.attrs
                    // .get(&Attrs::Style).map(|e| e.clone())}
                    // href = { &node.attrs.get(&Attrs::Href).map(|e| e.clone())}
                    // src = { &node.attrs.get(&Attrs::Src).map(|e| e.clone())}
                >

            { &node.text.clone()}
        </div>
    </span>
     }
}