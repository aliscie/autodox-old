use serde::{Deserialize, Serialize};
use yew::{Html, html};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Element, MouseEvent, window, Document};
use wasm_bindgen::{UnwrapThrowExt, JsCast};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub children: Vec<FileNode>,
}


pub trait TargetElement {
    fn target_element<'a>(&self)-> Option<Element>;

}

impl TargetElement for MouseEvent {
    fn target_element(&self) -> Option<Element> {
        // TODO
        //  improve this
        let doc = window().unwrap_throw().document().unwrap_throw();
        let x = self.page_x();
        let y = self.page_y();
        doc.element_from_point(x as f32, y as f32)
    }
}
