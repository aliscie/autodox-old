extern crate web_sys;

use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::{Dispatch, use_store};

use crate::element_tree::{Attrs, EditorElement, ElementTree};

use super::PasteConverter;


impl PasteConverter {
    pub fn parse_paste(editor: Rc<Element>) {
        let handle_paste = Closure::wrap(Box::new(move |_e: MouseEvent| {
            // let paste = (_e.clipboard_data() || window().clipboard_data()).get_data('text');
            log_1(&format!("onpaste {:?}", _e).into());
        }) as Box<dyn FnMut(_)>);

        &editor.add_event_listener_with_callback("paste", &handle_paste.as_ref().unchecked_ref());
        &handle_paste.forget();
    }
}
