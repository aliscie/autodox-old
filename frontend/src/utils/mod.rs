mod filetree;
pub use filetree::{FileNode, FileTree};


cfg_if::cfg_if!{
    if #[cfg(not(features = "web"))]{
        mod invoke;
        pub use invoke::{invoke, invoke_async};
    }
}

use web_sys::window;
pub fn alert<T : std::fmt::Debug>(message : &T){
    let window = window().unwrap();
    window.alert_with_message(&format!("{:?}", message)).unwrap();
}

mod get_title_bar;
pub use get_title_bar::get_titlebar;
