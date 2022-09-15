use yew::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent, window};
use wasm_bindgen::prelude::Closure;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub ondrop: Callback<DragEvent>,
}

#[function_component(DropUnder)]
pub fn drop_under(props: &Props) -> Html {
    let ondragenter_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr = _e.target_unchecked_into::<Element>();
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", "height: 20px; opacity:1;");
    });

    let _ondrop = props.ondrop.clone();
    let ondrop_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
        use web_sys::Element;
        let curr = _e.target_unchecked_into::<Element>();
        curr.set_attribute("style", " height: 5px; opacity:0;");
        // TODO call this here
        //  _ondrop(_e);

        // TODO
        //  get dragged item by dataframes
        //  do reorder the item or move it from parent to parent
        //     cases
        //     drag from place to place
        //     drag from paren to another paren
        //     drag from root to a parent..
    });


    let ondragleave_b: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_attribute("style", " height: 5px; opacity:0;");
    });

    html! {
    <div
           ondrop={ondrop_b}
           ondragenter={ondragenter_b}
           ondragleave={ondragleave_b}
           class="drag_under" />

     }
}
