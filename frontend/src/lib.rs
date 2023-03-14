extern crate futures;
extern crate shared;
extern crate wasm_bindgen_futures;
extern crate yew;

mod app;
mod backend;
mod components;
mod dummy_editor;
mod editor_components;
pub mod hooks;
pub mod pages;
mod router;
mod specific_components;
mod utils;
use dummy_editor::DummyEditor;

use app::App;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn run() {
//    yew::Renderer::<App>::new().render();
// }

#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<dummy_editor::DummyEditor>::new().render();
}
