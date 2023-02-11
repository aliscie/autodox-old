pub use device_info::DeviceInfo;
pub use get_title_bar::GetTitleBar;
pub use image::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::window;
mod device_info;
pub mod filetree;
mod get_title_bar;
mod image;

pub fn alert<T: std::fmt::Debug>(message: &T) {
    let window = window().unwrap();
    window
        .alert_with_message(&format!("{:?}", message))
        .unwrap();
}
