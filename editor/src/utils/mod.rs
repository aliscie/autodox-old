mod mouse_move;

// pub use mouse_move::on_mouse_move;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils/selection_popover.js")]
extern "C" {
    #[wasm_bindgen(js_name = my_function)]
    pub fn my_function() -> String;
}
