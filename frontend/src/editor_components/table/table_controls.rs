use super::table_context_menu::*;


use shared::schema::EditorElementCreate;
use std::collections::HashMap;

use editor::plugins::Position;
use shared::id::Id;
use shared::schema::{EditorChange, EditorElement};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlTableCellElement, HtmlInputElement, MouseEvent, window};
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
                    <ContextMenu items={vec![
                    html!{<a>{"+ add view"}</a>},
                    html!{<a>{"edite view"}</a>},
                    html!{<a>{"view formula"}</a>},]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state.render_context_menu.emit((mouse_event, element))
        })
    };
    let onclick_filter = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![
                    html!{<>
                    <a>
                    <input type="checkbox" id="vehicle2" name="vehicle2" value="Bike2"/>
                    <label for="vehicle2">{"age filter"}</label>
                    <span class="btn" style="color:tomato">{"delete"}</span>
                    </a>

                    <a>
                    <input type="checkbox" id="vehicle1" name="vehicle1" value="Bike2"/>
                    <label for="vehicle1">{"name filter"}</label>
                    <span class="btn" style="color:tomato">{"delete"}</span>
                    </a>

                    <a style="color:lightgreen"  class="btn">
                    {"+ add filter"}
                    </a>
                </>},
                    ]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            global_state.render_context_menu.emit((mouse_event, element));
        })
    };

    let on_filter_contextmenu = {
        let global_state = global_state.clone();
        Callback::from(move |mouse_event: MouseEvent| {
            let element = html! {
                <ContextProvider<GlobalEditorState> context = {global_state.clone()}>
                    <ContextMenu items={vec![
                    html!{<a>{"+ add filter"}</a>},
                    ]} event = {mouse_event.clone()}/ >
                </ContextProvider<GlobalEditorState>>
            };
            log!("filters clicked");
            global_state.render_context_menu.emit((mouse_event, element));
            log!("filters clicked 2");
        })
    };


    html! {
    <div class="table_title" >

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
            onclick={onclick_filter.clone()}
            oncontextmenu={on_filter_contextmenu.clone()}
            > {"filters"}
            </span>

    </div>
    }
}
