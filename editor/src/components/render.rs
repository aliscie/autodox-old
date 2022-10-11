use yew::prelude::*;
use yew::{function_component, html, Html, UseStateHandle};
use crate::element_tree::EditorElement;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, MouseEvent, window};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use shared::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[function_component(Render)]
pub fn render(props: &Props) -> Html {
    let node_ref = NodeRef::default();
    let display: UseStateHandle<&str> = use_state(|| "none");
    let left: UseStateHandle<String> = use_state(|| "left: 50px;".to_string());

    let node = props.node.clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let _left = left.clone();
    let _display = display.clone();
    let _node_ref = node_ref.clone();
    let handle_hovering = Closure::wrap(Box::new(move |_e: MouseEvent| {
        let element = _node_ref.clone().cast::<Element>();
        if element != None {
            let rec = element.unwrap().get_bounding_client_rect();
            let top = rec.top();
            let bottom = rec.bottom();
            let left = rec.left();
            let y = _e.client_y() as f64;

            // let d = *(_display.clone());
            // TODO prevent resending, and fix drag icon display.
            if y >= top && y <= bottom {
                // TODO position left does not seems to work
                _left.set(format!("left:{}px", left - 100 as f64).to_string());
                _display.set("inline-block")
            } else {
                _display.set("none")
            }
        }
    }) as Box<dyn FnMut(_)>);
    let _ = &doc.add_event_listener_with_callback("mousemove", &handle_hovering.as_ref().unchecked_ref());
    handle_hovering.forget();


    return html! {
    <span ref={node_ref}  >
        <div contenteditable="false"  style={format!("transition: 0.2s; position: absolute; display: {}; {:#?}",*(display.clone()),*(left.clone()))}>
            <i id="drag-icon" class="btn fa-solid fa-plus"></i>
            <i id="drag-icon" class="btn fa-solid fa-grip-dots-vertical"></i>
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
     };
}