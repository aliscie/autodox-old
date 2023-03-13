extern crate futures;
extern crate shared;
extern crate wasm_bindgen_futures;
extern crate yew;

mod app;
mod backend;
mod components;
pub mod hooks;
pub mod pages;
mod router;
mod specific_components;
mod utils;
mod dummy_editor;
pub mod editor_components;

use std::rc::Rc;
use dummy_editor::DummyEditor;

use app::App;
use wasm_bindgen::prelude::*;
use shared::schema::ElementTree;


#[derive(PartialEq, Clone)]
pub struct GlobalEditorState {
    pub element_tree: Rc<ElementTree>,
    // pub mutation: Callback<Vec<EditorChange>>,
    // pub render_context_menu: Callback<(MouseEvent, Html)>,
}

// #[wasm_bindgen]
// pub fn run() {
//     yew::Renderer::<App>::new().render();
// }


#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<dummy_editor::DummyEditor>::new().render();
}
