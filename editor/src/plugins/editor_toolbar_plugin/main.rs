use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    // pub on_select: Callback<>,
}

#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    let buttons = vec!["bold", "italic", "underscore", "comment", "droplet"];

    html! {<div contenteditable="false" id="selection-popper" class="buttons_group_class">
        {for buttons.iter().map(|button| html! {
            <span
            // onclick={props.on_select(&button)}
            class="btn"><i class={format!("fa-{}",&button)}></i></span>
        })}
            </div>}
}