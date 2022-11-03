extern crate web_sys;

use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::prelude::*;

mod trigger_paste;

#[wasm_bindgen(module = "/src/plugins/paste_converter/fetch_pasted.js")]
extern "C" {
    #[wasm_bindgen(js_name = my_function)]
    pub fn fetch_pasted(editor: Element) -> String;
    // pub fn fetch_pasted() -> Json;
}

pub struct PasteConverter {
    pub(crate) drag_icon_width: f32,
    pub(crate) doc: web_sys::Document,
    pub(crate) body: Element,
    pub(crate) editor: Rc<Element>,
    prev: Option<Element>,
    pub(crate) curr: Option<Element>,
    dragged: Option<Element>,
}

impl PasteConverter {
    pub(crate) fn new(editor: Rc<Element>) {
        PasteConverter::parse_paste(editor);
    }
}
