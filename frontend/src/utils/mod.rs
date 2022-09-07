mod filetree;
pub use filetree::{FileNode, FileTree};

use web_sys::window;
pub fn alert<T : std::fmt::Debug>(message : &T){
    let window = window().unwrap();
    window.alert_with_message(&format!("{:?}", message)).unwrap();
}

mod get_title_bar;
pub use get_title_bar::get_titlebar;

mod mouse_move;
pub use mouse_move::on_mouse_move;
