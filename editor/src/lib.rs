extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;

// this is used for the work space
mod app;
mod plugins;
mod utils;
pub use app::Editor;
pub(crate) mod app_components;
pub(crate) mod components;
mod element_tree;
