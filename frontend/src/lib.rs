extern crate futures;
extern crate shared;
extern crate wasm_bindgen_futures;
extern crate yew;

use std::env;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use app::App;

mod app;
mod specific_components;
mod backend;
mod components;
mod router;
mod test;
mod utils;


lazy_static! {
    pub static ref IS_WEB: bool = cfg!(feature = "web");
}

#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<App>::new().render();
}
