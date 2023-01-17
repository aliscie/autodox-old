use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{Element, KeyboardEvent, Node, window};

// use shared::log;

#[wasm_bindgen(module = "/src/plugins/inset_component/caret_position.js")]
extern "C" {
    #[wasm_bindgen(js_name = getCaretPosition)]
    pub fn get_caret_position() -> u32;
}


pub fn insert_components(editor: &Element, trigger: String) {
    let mut text_value: String = "".to_string();
    let mut track: bool = false;
    let mut range = window().unwrap_throw().document().unwrap_throw().create_range().unwrap();
    let mut i = 0;
    let mut f = 0;

    let handle_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let selection: Node = window().unwrap_throw().get_selection().unwrap().unwrap().focus_node().unwrap();

        if track && &e.key() == "Enter" || e.key() == "Tab" {
            e.prevent_default();
            track = false;

            f = get_caret_position();
            range.set_start(&selection, i);
            range.set_end(&selection, f);
            range.delete_contents();
            i = 0;
            f = 0;
        }

        if &e.key() == "Enter" {
            track = false;
        }

        if track {
            text_value = format!("{}{}", text_value, e.key()).into();
            // log!(&text_value);
        }

        if e.key() == trigger {
            track = true;
            i = get_caret_position();
        }
    }) as Box<dyn FnMut(_)>);

    &editor.add_event_listener_with_callback("keydown", &handle_keydown.as_ref().unchecked_ref());
    &handle_keydown.forget();
}