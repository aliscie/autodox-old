use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

pub use device_info::DeviceInfo;
pub use get_title_bar::get_titlebar;

pub mod filetree;

pub fn alert<T: std::fmt::Debug>(message: &T) {
    let window = window().unwrap();
    window
        .alert_with_message(&format!("{:?}", message))
        .unwrap();
}

mod get_title_bar;
mod device_info;

#[wasm_bindgen(module = "/src/utils/ic_agent.js")]
extern "C" {
    #[wasm_bindgen(js_name = read)]
    pub async fn read(canister_id: String) -> JsValue;
    
    #[wasm_bindgen(js_name = createActor)]
    pub async fn createActor(canister_id: String) -> JsValue;
}


