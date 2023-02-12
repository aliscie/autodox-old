use crate::plugins;
use crate::plugins::inset_component::types::Position;
use crate::plugins::DropDownItem;
use shared::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, Element, KeyboardEvent, Node, Range};
use yew::suspense::{SuspensionResult, UseFutureHandle};
use yew::{Callback, UseStateHandle};

use crate::backend;
use shared::schema::UserQuery;
use shared::schema::{FileDirectory, FileNode};
use shared::*;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::suspense::*;
use yewdux::functional::use_store;
use yewdux::prelude::*;

#[hook]
pub fn use_trigger_popover(
    trigger: String,
    command: Callback<Range>,
) -> (UseStateHandle<Option<Range>>, UseStateHandle<Option<Position>>) {
    // pub fn trigger_popover(editor: &Element, trigger: String, position: UseStateHandle<Option<Position>>, input_text: UseStateHandle<String>, command: Callback<Range>) {

    let position: UseStateHandle<Option<Position>> = use_state(|| None);
    // let input_text: UseStateHandle<String> = use_state(|| "".to_string());
    let range_state: UseStateHandle<Option<Range>> = use_state(|| None);

    // let _input_text = input_text.clone();
    let _position = position.clone();

    let handle_click = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        // is_track = false;
        if (*_position).is_some() {
            _position.set(None);
        }
    }) as Box<dyn FnMut(_)>);
    &window()
        .unwrap_throw()
        .add_event_listener_with_callback("click", &handle_click.as_ref().unchecked_ref());
    &handle_click.forget();

    // let _input_text = input_text.clone();
    let _range_state = range_state.clone();
    let _position = position.clone();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let editor = doc.query_selector(".text_editor");
    let _editor = editor.clone();
    use_effect_with_deps(
        move |editor_ref| {
            let position = _position.clone();

            let mut is_track = (*(_position.clone())).is_some();
            let mut range = window()
                .unwrap_throw()
                .document()
                .unwrap_throw()
                .create_range()
                .unwrap();
            let _range = range.clone();

            let _position = position.clone();
            let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
                let curr_focus: Node = selection.focus_node().unwrap();
                let _ = range.set_end(&curr_focus, plugins::get_caret_position());

                if is_track && (&e.key() == "Enter" || e.key() == "Tab") {
                    e.prevent_default();
                    // let r = range.delete_contents();
                    is_track = false;
                    command.emit(range.clone());
                    _position.set(None);
                } else if e.key() == trigger {
                    let _ = range.set_start(&curr_focus, plugins::get_caret_position());
                    if let Some(p) = range.get_client_rects() {
                        let p = p.get(0).unwrap();
                        is_track = true;
                        _position.set(Some(Position {
                            x: p.right() + 10 as f64,
                            y: p.y(),
                        }));
                    };
                } else if is_track && &e.key() != "Enter" && e.key() != "Tab" {
                    _range_state.set(Some(range.clone()));
                    log!(format!("{}", range.to_string()).to_string());
                    // _input_text.set(format!("{}", range.to_string()).to_string());
                    if let Some(p) = range.get_client_rects() {
                        let p = p.get(0).unwrap();
                        is_track = true;
                        _position.set(Some(Position {
                            x: p.right() + 10 as f64,
                            y: p.y(),
                        }));
                    };
                }
            }) as Box<dyn FnMut(_)>);
            if let Some(editor) = _editor.unwrap() {
                let _ = &editor.add_event_listener_with_callback(
                    "keydown",
                    &handle_keydown.as_ref().unchecked_ref(),
                );
            };
            move || {
                drop(handle_keydown);
            }
        },
        editor.clone(),
    );
    return (range_state, position);
}
