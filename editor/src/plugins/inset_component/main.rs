use std::collections::btree_map::Range;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, KeyboardEvent, Node, window};
use shared::log;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/plugins/inset_component/caret_position.js")]
extern "C" {
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position() -> u32;
}


pub fn insert_components(editor: &Element, trigger: String) {
    let doc = window().unwrap_throw().document().unwrap_throw();
    let mut text_value: String = "".to_string();
    let mut track: bool = false;
    let _editor = &doc.query_selector("#text_editor").unwrap().unwrap().query_selector("p").unwrap().unwrap();
    let _editor = _editor.clone();
    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();
    let mut i = 0;
    let mut f = 0;

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        if track && &e.key() == "Enter" || e.key() == "Tab" {
            f = get_caret_position();

            range.set_start(_editor.first_child().unwrap().as_ref(), i);
            range.set_end(_editor.first_child().unwrap().as_ref(), f);
            range.delete_contents();
            i = 0;
            f = 0;
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
            i = get_caret_position();
        }
    }) as Box<dyn FnMut(_)>);

    &editor.add_event_listener_with_callback("keydown", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}