use web_sys::{Element, KeyboardEvent, Node, window};
use yew::UseStateHandle;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use crate::plugins;
use crate::plugins::inset_component::types::Position;
use shared::*;

pub fn trigger_popover(editor: &Element, trigger: String, position: UseStateHandle<Option<Position>>, input_text: UseStateHandle<String>) {
    let mut track: bool = false;
    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();

    let _position = position.clone();
    let _range = range.clone();

    let handle_click = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        track = false;
        _position.set(None);
    }) as Box<dyn FnMut(_)>);

    &window().unwrap_throw().add_event_listener_with_callback("click", &handle_click.as_ref().unchecked_ref());
    &handle_click.forget();

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let selection = window().unwrap_throw().get_selection().unwrap().unwrap();
        let curr_focus: Node = selection.focus_node().unwrap();
        if track && (&e.key() == "Enter" || e.key() == "Tab") {
            e.prevent_default();
            range.set_end(&curr_focus, plugins::get_caret_position()).unwrap();
            range.delete_contents();
            position.set(None);
            track = false;
            e.prevent_default();
        }

        if e.key() == trigger {
            range.set_start(&curr_focus, plugins::get_caret_position());
            range.set_end(&curr_focus, plugins::get_caret_position());
            track = true;
        }

        if track {
            range.set_end(&curr_focus, plugins::get_caret_position()).unwrap();
            let p = range.get_client_rects().unwrap().get(0);
            input_text.set(format!("{}", range.to_string()).to_string());

            if let Some(p) = p {
                position.set(Some(Position { x: p.right() + 10 as f64, y: p.y() }));
            };
        }
    }) as Box<dyn FnMut(_)>);


    &editor.add_event_listener_with_callback("keydown", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}
