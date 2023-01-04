use markup5ever_rcdom::Node;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, KeyboardEvent};
use shared::log;

pub fn insert_components(editor: &Element, trigger: String) {
    let mut text_value: String = "".to_string();
    let mut track: bool = false;

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        // let node: &Node = editor.into();
        // let mut range = None;


        if track && &e.key() == "Enter" || e.key() == "Tab" {
            // range.move_to_current();
            // range.select();
            // range.delete();
        }
        if &e.key() == "Tab" {
            track = false;
            &e.prevent_default();
        }

        if &e.key() == "Enter" {
            track = false;
        }

        if track {
            text_value = format!("{}{}", text_value, e.key()).into();
            log!(&text_value);
        }

        if e.key() == trigger {
            track = true;
            // range = Some(node.create_text_range())
        }
    }) as Box<dyn FnMut(_)>);

    &editor.add_event_listener_with_callback("keydown", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}