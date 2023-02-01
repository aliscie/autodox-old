use std::fmt::Display;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, HtmlDocument};
use crate::plugins::editor_toolbar;
use shared::*;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;
use web_sys::Selection;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub action: fn(String, Selection),
}


#[function_component]
pub fn EditorToolbar(props: &Props) -> Html {
    let init: UseStateHandle<bool> = use_state(|| false);

    let win = web_sys::window().unwrap();
    let document = win.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let editor = win.document().unwrap().query_selector(".text_editor").unwrap();
    let _editor = editor.clone();

    let _init = init.clone();
    let handle_click = Closure::wrap(Box::new(move |e: MouseEvent| {
        if !(*_init) { _init.set(true) }
    }) as Box<dyn FnMut(_)>);

    &html_document.add_event_listener_with_callback("click", &handle_click.as_ref().unchecked_ref());
    &handle_click.forget();

    use_effect_with_deps(
        move |editor_ref| {
            if let Some(text_editor) = _editor {
                let _toolbar_action = editor_toolbar();
            }
        },
        init.clone(),
    );

    let buttons = [
        "Bold",
        "Italic",
        "Underline",
        "Comment",
        "Droplet",
        "Color",
    ];

    // let action = props.action.clone();
    // let _action = props.action.clone();
    let _html_document = html_document.clone();
    let oninput: Callback<InputEvent> = Callback::from(move |_e: InputEvent| {
        _e.prevent_default();
        let x = _html_document.exec_command_with_show_ui_and_value("foreColor", false, "rgba(0,0,0,0.5)").unwrap();
        if let Some(data) = _e.data() {
            // TODO why thi is not working? (not this is a compensatory task)
            // let x = html_document.exec_command_with_show_ui_and_value("foreColor", false, data).unwrap();
            // _action.emit(data)
        }
    });

    html! {
        <span class={css_file_macro!("toolbar.css")}>
            <div
            contenteditable="false" id="selection-popper" class="buttons_group_class">
            {buttons.into_iter().map(|button|{
                let icon = format!("{}",button).to_string().to_lowercase();
                if icon == "color"{
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
