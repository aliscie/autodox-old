mod filenode;
mod invoke;
pub use filenode::{FileNode, FileNavigableNode};
pub use invoke::{invoke, invoke_async};
use web_sys::window;


pub fn alert<T : std::fmt::Debug>(message : &T){
    let window = window().unwrap();
    window.alert_with_message(&format!("{:?}", message)).unwrap();
}
