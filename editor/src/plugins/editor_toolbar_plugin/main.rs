use std::fmt::Display;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, HtmlDocument};
use crate::plugins::editor_toolbar;
use shared::log;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;

use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: Callback<String>,
}


#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();

    use_effect_with_deps(
        move |editor_ref| {
            let _toolbar_action = editor_toolbar();
        },
        (),
    );

    let buttons = [
        "Bold",
        "Italic",
        "Underline",
        "Comment",
        "Droplet",
        "Color",
    ];

    let action = props.action.clone();
    let _action = props.action.clone();
    let _html_document = html_document.clone();
    let oninput: Callback<InputEvent> = Callback::from(move |_e: InputEvent| {
        _e.prevent_default();
        let x = _html_document.exec_command_with_show_ui_and_value("foreColor", false, "rgba(0,0,0,0.5)").unwrap();
        if let Some(data) = _e.data() {
            // TODO why thi is not working? (not this is a compensatory task)
            // let x = html_document.exec_command_with_show_ui_and_value("foreColor", false, data).unwrap();
            _action.emit(data)
        }
    });

    html! {
    <div contenteditable="false" id="selection-popper" class="buttons_group_class">

        {buttons.into_iter().map(|button|{
            let icon = format!("{}",button).to_string().to_lowercase();
            if icon == "color"{
                return html!{<input  oninput={oninput.clone()} type="color" value="#f6f82" id="colorPicker"/>}
            } else {
                html! {
                <button
                    onmousedown = {
                        let action = action.clone();
                        let html_document = html_document.clone();
                        let icon = icon.clone();
                        move |_| {
                            let x = html_document.exec_command(&icon).unwrap();
                            action.clone().emit(button.to_string())
                        }
                        }
                    class={format!("fa-solid fa-{}",icon)}
                ></button>
                }
            }


        }).collect::<Html>()}
        </div>
    }
}
