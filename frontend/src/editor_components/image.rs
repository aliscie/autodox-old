use shared::schema::EditorElementCreate;
use std::collections::HashMap;

use editor::plugins::Position;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, DragEvent, MouseEvent};
use yew::prelude::*;
use yew::{function_component, html};
use yew_hooks::prelude::*;

use editor::GlobalEditorState;
use crate::components::PopOverMenu;
use web_sys::{Element, HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub columns: Vec<String>,
    // pub rows: Vec<String>,
    pub node: EditorElement,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Image(props: &Props) -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let oncontextmenu = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            position.set(Some(e));
        })
    };
    let click_float = {
        Callback::from(move |e: MouseEvent| {
            // TODO // props.node.attrs.insert("style".to_string(), "float: left;".to_string());
        })
    };
    let items: Vec<Html> = vec![
        html! {<a
            onclick={click_float}
            ><i class="fa-solid fa-paragraph"></i>{"Float"}</a>},
        html! {<a href={"blob:https://mega.nz/9b38ed11-8f43-404b-87d5-eb7b2ac37692"} target="_blank"><i class="fa-solid fa-align-slash"></i>{"block"}</a>},
        html! {<a><i class="fa-solid fa-outdent"></i>{"sharp"}</a>},
    ];

    let mut src = props.node.attrs.get("src").unwrap();
    let mut style = "border: 2px solid; padding: 20px; width: 300px; resize: both; overflow: auto;".to_string();
    if let Some(s) = props.node.attrs.get("style") {
        style = format!("{} {}", &style, s).to_string();
    }
    // let ondragstart: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     let curr: Element = _e.target_unchecked_into();
    //     curr.set_attribute("style", "height:50px;").unwrap_throw();
    // });
    return html! {
        <span
        // {ondragstart}
        draggable="true" {oncontextmenu} {style} >
        <PopOverMenu  {items} position = {position.clone()}/>
        {props.node.content.clone()}
        <img draggable="false" src={src.to_string()}/>
        </span>
    };
}
