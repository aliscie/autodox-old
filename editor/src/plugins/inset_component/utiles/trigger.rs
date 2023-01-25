use web_sys::{Element, KeyboardEvent, Node, window};
use yew::UseStateHandle;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use crate::plugins;
use crate::plugins::inset_component::types::Position;
use shared::*;
use crate::plugins::DropDownItem;

pub fn trigger_popover(editor: &Element, trigger: String, position: UseStateHandle<Option<Position>>, input_text: UseStateHandle<String>) {
    let mut is_track = (*(position.clone())).is_some();

    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();
    let _position = position.clone();
    let _range = range.clone();

    let handle_click = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        is_track = false;
        _position.set(None);
    }) as Box<dyn FnMut(_)>);

    &window().unwrap_throw().add_event_listener_with_callback("click", &handle_click.as_ref().unchecked_ref());
    &handle_click.forget();

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
        let curr_focus: Node = selection.focus_node().unwrap();
        let _ = range.set_end(&curr_focus, plugins::get_caret_position());

        if is_track && (&e.key() == "Enter" || e.key() == "Tab") {
            e.prevent_default();
            let r = range.delete_contents();
            is_track = false;
            position.set(None);
        };

        if e.key() == trigger {
            let _ = range.set_start(&curr_focus, plugins::get_caret_position());
            if let Some(p) = range.get_client_rects() {
                let p = p.get(0).unwrap();
                is_track = true;
                position.set(Some(Position { x: p.right() + 10 as f64, y: p.y() }));
            };
        }

        if is_track && &e.key() != "Enter" && e.key() != "Tab" {
            log!("range");
            input_text.set(format!("{}", range.to_string()).to_string());
            if let Some(p) = range.get_client_rects() {
                let p = p.get(0).unwrap();
                is_track = true;
                position.set(Some(Position { x: p.right() + 10 as f64, y: p.y() }));
            };
        }
    }) as Box<dyn FnMut(_)>);


    &editor.add_event_listener_with_callback("keydown", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}
