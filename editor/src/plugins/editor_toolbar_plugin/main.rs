use web_sys::HtmlInputElement;
use yew::prelude::*;
use shared::log;
use crate::plugins::editor_toolbar;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: Callback<MouseEvent>,
}

#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    use_effect_with_deps(
        move |editor_ref| {
            let _toolbar_action = editor_toolbar();
        }, ());

    let buttons = vec!["bold", "italic", "underscore", "comment", "droplet"];

    html! {<div contenteditable="false" id="selection-popper" class="buttons_group_class">

        {for buttons.iter().map(|button| html! {
            <button
            onmousedown={props.action.clone()}
            ><i class={format!("fa-{}",&button)}></i>{&button.chars().nth(0).unwrap()}</button>
        })}
            </div>}
}