use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub position: String,
}

#[function_component]
pub fn Drag(props: &Props) -> Html {
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {});
    let p = props.position.clone();
    // let items: Vec<Html> = vec![
    //     html! {<><i class="fa-solid fa-user"></i>{"Convert component"}</>},
    //     html! {<><i class="fa-solid fa-eye"></i>{"Copy"}</>},
    // ];

    html! { <>
    // <Menu
    //     event={position.clone()}
    //      {items}
    //   />
    <div  style={format!("transition: 0.2s; position: absolute; {};",p)}>
            <i contenteditable="false" id="drag-icon" style="display: inline-block;">{"+"}</i>
            <i contenteditable="false" id="drag-icon" style="display: inline-block;">{"::"}</i>
    </div>
     </>}
}
