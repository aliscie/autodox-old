use crate::components::PopOverMenu;

use super::table_context_menu::*;

use shared::schema::EditorElementCreate;
use std::collections::HashMap;

use editor::plugins::Position;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, HtmlInputElement, HtmlTableCellElement, MouseEvent};
use yew::prelude::*;
use yew::{function_component, html};
use yew_hooks::prelude::*;

use editor::GlobalEditorState;
use shared::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn TableControls(props: &Props) -> Html {
    let global_state = use_context::<GlobalEditorState>().expect("cannot access global state");

    let on_view_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <a>{"+ add view"}</a>
                    <a>{"edite view"}</a>
                    <a>{"view formula"}</a>
                </ContextProvider<GlobalEditorState>>
            };
            global_state
                .render_context_menu
                .emit((mouse_event, element))
        })
    };

    let on_filter_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <a>{"+ add filter"}</a>
                </ContextProvider<GlobalEditorState>>
            };
            global_state
                .render_context_menu
                .emit((mouse_event, element))
        })
    };

    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let onclick_filters = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
        })
    };
    let filters_items = vec![
        html! {<a>
            <input type="checkbox" id="vehicle2" name="vehicle2" value="Bike2"/>
            <label for="vehicle2">{"age filter"}</label>
        </a>},
        html! {<a>
            <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike2"/>
            <label for="vehicle1">{"name filter"}</label>
        </a>},
    ];

    html! {
    <div class="table_title" >
            <PopOverMenu click_on={Some(true)} items = {filters_items} position = {position.clone()}/>

            <h3 contenteditable="true"  style="display:inline-block; margin-right: 5px"> {"table title"}</h3>

            <select style="margin-right: 5px"
            oncontextmenu={on_view_contextmenu.clone()}
            >
                <option>{"gallery view"}</option>
                <option>{"grid view"}</option>
            </select>

            <span
            style="-webkit-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;"
            contenteditable="false"
            onclick={onclick_filters}
            oncontextmenu={on_filter_contextmenu.clone()}
            > {"filters"}
            </span>

    </div>
    }
}
