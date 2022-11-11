use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, MouseEvent, window, KeyboardEvent, Event};
use yew::{function_component, html, UseStateHandle};
use yew::prelude::*;
use shared::log;

use crate::components::Drag;
use crate::element_tree::EditorElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[function_component(Render)]
pub fn render(props: &Props) -> Html {
    let node_ref = NodeRef::default();
    let position: UseStateHandle<String> = use_state(|| "display:none".to_string());
    let node = props.node.clone();
    let doc = window().unwrap_throw().document().unwrap_throw();

    let _position = position.clone();
    let _node_ref = node_ref.clone();
    let handle_hovering = Closure::wrap(Box::new(move |_e: MouseEvent| {
        let element = _node_ref.clone().cast::<Element>();
        if element.is_some() {
            let rec = element.unwrap().get_bounding_client_rect();
            let top = rec.top();
            let left = rec.left();
            let bottom = rec.bottom();
            let y = _e.client_y() as f64;
            // TODO prevent too many re-rendering
            if y >= top && y <= bottom {
                _position.set(format!(
                    "display:inline-block; left:{}px",
                    left - (25 as f64)
                ))
            } else {
                _position.set("display:none".to_string())
            }
        }
    }) as Box<dyn FnMut(_)>);
    let _ = &doc
        .query_selector(".text_editor_container")
        .unwrap()
        .unwrap()
        .add_event_listener_with_callback("mousemove", &handle_hovering.as_ref().unchecked_ref());
    handle_hovering.forget();

    // let onkeydown = Closure::wrap(Box::new(move |_e: KeyboardEvent| {
    //     log!(_e)
    // }) as Box<dyn FnMut(_)>);

    // TODO user posting identifiers like id...
    let onkeydown = Callback::from(move |e: KeyboardEvent| {
        // TODO I don't know why this is not working
        log!(e);
        // TODO
        //  log!(e.target().unwrap_throw().value());
        //  log!(let curr: Element = e.target_unchecked_into();curr.inner_html());
    });

    let onchange = Callback::from(move |e: Event| {
        // TODO I don't know why this is not working
        log!(e);
        // TODO
        //  log!(e.target().unwrap_throw().value());
        //  log!(let curr: Element = e.target_unchecked_into();curr.inner_html());
    });

    html! {
    <span ref={node_ref}  >
             <Drag position={(*(position.clone())).to_string()}/>
        <div
        {onkeydown}
        {onchange}
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
