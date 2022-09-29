use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = say_hello)]
    pub fn say_hello() -> String;
}