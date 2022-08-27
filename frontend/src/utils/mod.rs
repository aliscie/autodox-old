mod filenode;
pub use filenode::{FileNode, FileNavigableNode, FileMap};


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
