// here we create general specific_components that are reusable by anyapp
// Don't import anything this folder from outside.

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::prelude;
pub use files::*;
pub use main::*;
pub use element::*;

mod files;
mod main;
mod element;


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

    // #[wasm_bindgen(js_name = register)]
    // pub async fn register(username: String) -> JsValue;
    //
    // #[wasm_bindgen(js_name = test_ic)]
    // pub async fn test_ic() -> JsValue;

    #[wasm_bindgen(js_name = test_connect_wasm_bindgen)]
    pub async fn test_wasm() -> JsValue;

    #[wasm_bindgen(js_name = update_profile)]
    pub async fn update_profile(image: Vec<u8>) -> JsValue;

    #[wasm_bindgen(js_name = get_profile)]
    pub async fn get_profile() -> JsValue;

    // #[wasm_bindgen(js_name = create_directory)]
    // pub async fn create_directory_ic() -> JsValue;


    // #[wasm_bindgen(js_name = create_file)]
    // pub async fn create_file_ic(text: String) -> JsValue;

    // #[wasm_bindgen(js_name = read)]
    // pub async fn read(canister_id: String) -> JsValue;

    // #[wasm_bindgen(js_name = createActor)]
    // pub async fn createActor() -> JsValue;
}


