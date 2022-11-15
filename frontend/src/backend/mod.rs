// here we create general app_components that are reusable by anyapp
// Don't import anything this folder from outside.

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::prelude;
pub use files::*;
pub use main::*;

mod files;
mod main;


#[wasm_bindgen(module = "/src/backend/ic_agent.js")]
extern "C" {
    #[wasm_bindgen(js_name = call_actor)]
    pub async fn call_actor(canister_id: String, method: String, args: Option<String>) -> JsValue;
}

