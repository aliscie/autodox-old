extern crate web_sys;

use yew::prelude::*;

mod app;


mod element_tree;

// this is used for the work space
mod utils;
mod plugins;

pub use app::Editor;


// this is used for the work isolated development
fn main() {
    // yew::start_app::<Editor>();
//TODO
// 19  |     yew::start_app::<Editor>();
// |                      ^^^^^^ the trait `std::default::Default` is not implemented for `Props`
}
