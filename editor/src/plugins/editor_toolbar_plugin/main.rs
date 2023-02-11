use crate::plugins::editor_toolbar;
use shared::*;
use std::fmt::Display;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Selection;
use web_sys::{window, HtmlInputElement};
use web_sys::{Element, HtmlDocument};
use yew::prelude::*;
use yew_hooks::use_event_with_window;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub editor_ref: NodeRef,
    // pub command: Callback<(String, Selectoin), Option<()>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ToolbarButton {
    Bold,
    Italic,
    Underline,
    Comment,
    Droplet,
    Color,
}

use ToolbarButton::*;

#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    let init: UseStateHandle<bool> = use_state(|| false);
    let editor_ref = props.editor_ref.clone();
    {
        let init = init.clone();
        use_event_with_window("click", move |e: MouseEvent| {
            init.set(true);
        });
    }
    use_effect_with_deps(
        move |_init| {
            if let Some(text_editor) = editor_ref.get().and_then(|f| f.parent_element()) {
                let _toolbar_action = editor_toolbar();
            }
        },
        init.clone(),
    );

    let buttons = [Bold, Italic, Underline, Comment, Droplet, Color];
    let document = window().unwrap().document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();

    // let action = props.action.clone();
    // let _action = props.action.clone();
    let _html_document = html_document.clone();
    let oninput: Callback<InputEvent> = Callback::from(move |_e: InputEvent| {
        _e.prevent_default();
        let x = _html_document
            .exec_command_with_show_ui_and_value("foreColor", false, "rgba(0,0,0,0.5)")
            .unwrap();
        if let Some(data) = _e.data() {
            // TODO why thi is not working? (not this is a compensatory task)
            // let x = html_document.exec_command_with_show_ui_and_value("foreColor", false, data).unwrap();
            // _action.emit(data)
        }
    });

    // let command = props.command.clone();

    html! {
        <span class={css_file_macro!("toolbar.css")}>
            <div
            contenteditable="false" id="selection-popper" class="buttons_group_class">
            {buttons.into_iter().map(|button|{
                let icon = format!("{:?}",button).to_string().to_lowercase();
                if button == Color {
                    return html!{<input  oninput={oninput.clone()} type="color" value="#f6f82" id="colorPicker"/>}
                } else {
                    html! {
                    <span
                        onmousedown = {
                            // let action = action.clone();
                            let html_document = html_document.clone();
                            let icon = icon.clone();
                            move |_| {
                                // let selection: Selection = window().unwrap_throw().get_selection().unwrap().unwrap();
                                let _ = html_document.exec_command(&icon).unwrap();
                                // action(button.to_string(),selection)
                                // command.emit(button.to_string(),selection)
                            }
                            }
                        class={format!("btn fa-solid fa-{}",icon)}
                    ></span>
                    }
                }
            }).collect::<Html>()}
            </div>
        </span>

    }
}
