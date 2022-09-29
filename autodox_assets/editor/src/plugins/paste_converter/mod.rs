extern crate web_sys;

mod trigger_paste;

use std::rc::Rc;
use web_sys::{DragEvent, Element, MouseEvent, window};

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;


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
        // TODO optimize this
        //     let drag_icon_width = 20 as f32;
        //     let doc = window().unwrap_throw().document().unwrap_throw();
        //     let body = doc.query_selector("body").unwrap_throw().unwrap_throw();
        //     let _self = &mut PasteConverter { editor, drag_icon_width, doc, body, prev: None, curr: None, dragged: None };
        PasteConverter::parse_paste(editor);
    }
}

