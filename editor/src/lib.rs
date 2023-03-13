extern crate web_sys;
extern crate yew;

pub use app::*;

mod app;
mod backend;
pub(crate) mod components;
mod handle_mutation;
pub mod insertion_closures;
pub mod plugins;
pub mod render;
pub(crate) mod spesific_components;
mod utils;
