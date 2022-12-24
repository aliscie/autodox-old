// mod mouse_move;

// pub use mouse_move::on_mouse_move;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/plugins/editor_toolbar_plugin/editor_toolbar.js")]
extern "C" {
    #[wasm_bindgen(js_name = editor_toolbar)]
    pub fn editor_toolbar() -> String;
}
