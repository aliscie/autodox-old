// here we create general specific_components that are reusable by any app
// Don't import anything this folder from outside.

pub use element::*;
pub use files::*;
pub use main::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
mod element;
mod files;
mod main;

#[wasm_bindgen(module = "/src/backend/ic_agent.js")]
extern "C" {
    #[wasm_bindgen(js_name = identify)]
    pub async fn identify() -> JsValue;

    #[wasm_bindgen(js_name = logout)]
    pub async fn logout();

    #[wasm_bindgen(js_name = is_logged)]
    pub async fn is_logged() -> JsValue;

    #[wasm_bindgen(js_name = register)]
    pub async fn register(username: String) -> JsValue;

    #[wasm_bindgen(js_name = update_profile)]
    pub async fn update_profile(data: String) -> JsValue;

    #[wasm_bindgen(js_name = get_profile)]
    pub async fn get_profile() -> JsValue;

    #[wasm_bindgen(js_name = get_directories)]
    pub async fn get_directories_ic() -> JsValue;

    #[wasm_bindgen(js_name = call_ic)]
    pub async fn call_ic(method: String, stringify: String) -> JsValue;

    #[wasm_bindgen(js_name = call_ic_np)]
    pub async fn call_ic_np(method: String) -> JsValue; // np: no parameter
}
