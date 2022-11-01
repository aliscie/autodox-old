pub mod filetree;
use web_sys::window;
pub fn alert<T: std::fmt::Debug>(message: &T) {
    let window = window().unwrap();
    window
        .alert_with_message(&format!("{:?}", message))
        .unwrap();
}

mod get_title_bar;
pub use get_title_bar::get_titlebar;

mod device_info;
pub use device_info::DeviceInfo;
