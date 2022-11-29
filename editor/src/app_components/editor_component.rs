use yew::{function_component, html};
use yew::prelude::*;

use shared::schema::EditorElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub node: EditorElement,
}

#[function_component(EditorComponent)]
pub fn editor_component(props: &Props) -> Html {
    //let node_ref = NodeRef::default();
    //let position: UseStateHandle<String> = use_state(|| "display:none".to_string());
    let node = &props.node;
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

    //let _position = position.clone();
    //let _node_ref = node_ref.clone();
    //let handle_hovering = Closure::wrap(Box::new(move |_e: MouseEvent| {
        //let element = _node_ref.clone().cast::<Element>();
        //if element.is_some() {
            //let rec = element.unwrap().get_bounding_client_rect();
            //let top = rec.top();
            //let left = rec.left();
            //let bottom = rec.bottom();
            //let y = _e.client_y() as f64;
            //// TODO prevent too many re-rendering
            //if y >= top && y <= bottom {
                //_position.set(format!(
                    //"display:inline-block; left:{}px",
                    //left - (25 as f64)
                //))
            //} else {
                //_position.set("display:none".to_string())
            //}
        //}
    //}) as Box<dyn FnMut(_)>);
    //let _ = &doc
        //.get_element_by_id("text_editor_container")
        //.unwrap()
        //.add_event_listener_with_callback("mousemove", &handle_hovering.as_ref().unchecked_ref());
    //handle_hovering.forget();
    html! {
    //<span ref={node_ref}  >
             //<Drag position={format!("{}",*(position.clone()))}/>
        <div
        id = { node.id.to_string() }

                    // style = { &node.attrs
                    // .get(&Attrs::Style).map(|e| e.clone())}
                    // href = { &node.attrs.get(&Attrs::Href).map(|e| e.clone())}
                    // src = { &node.attrs.get(&Attrs::Src).map(|e| e.clone())}
                >

            { &node.text }
        </div>
    //</span>
     }
}
