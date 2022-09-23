extern crate web_sys;

use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, KeyboardEvent, MouseEvent, window};
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::{Dispatch, use_store};

use crate::element_tree::{Attrs, EditorElement, ElementTree};

use super::PasteConverter;
use crate::plugins::main::PluginTraits;


impl PasteConverter {
    pub fn parse_paste(editor: Rc<Element>) {

        // TODO in addition to rendering we will have conversion
        //   for example when a use copy and paste something from the internet we will have conversion algorithm like this
        //   https://share.descript.com/view/xR05MPWVQG5
        //   changes.items().map(|item|,{
        //   the `to_nodes()` method should handle item.tag, item.attribute, item.value, item.children
        //   if item.children != None { item.children.unwrap().map(|child|,{  self.to_nodes(child) }) }
        //   let node = element_tree_dispatch.get().to_nodes(item)
        //   })

        let handle_paste = Closure::wrap(Box::new(move |_e: KeyboardEvent| {
            // let paste = (_e.clipboard_data() || window().clipboard_data()).get_data('text');
            log_1(&format!("onpaste {:?}", _e).into());
        }) as Box<dyn FnMut(_)>);

        &editor.add_event_listener_with_callback("paste", &handle_paste.as_ref().unchecked_ref());
        &handle_paste.forget();
    }
}
impl PluginTraits for PasteConverter {}