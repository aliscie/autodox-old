extern crate futures;
extern crate shared;
extern crate wasm_bindgen_futures;
extern crate yew;
pub mod editor_components;
mod app;
mod backend;
mod components;
pub mod hooks;
pub mod pages;
mod router;
mod specific_components;
mod utils;
use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<App>::new().render();
}
