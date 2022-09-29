extern crate web_sys;
use yew::prelude::*;

mod app;
use app::Editor;
mod element_tree;
// this is used for the work isolated development
fn main() {
    yew::start_app::<Editor>();
}
