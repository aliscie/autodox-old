// use std::rc::Rc;

//
use wasm_bindgen::prelude::*;

//
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = say_hello, catch)]
    pub fn say_hello() -> Result<JsValue, JsValue>;
}
