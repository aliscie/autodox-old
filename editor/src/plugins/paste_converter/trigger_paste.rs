extern crate web_sys;

use std::{borrow::Borrow, collections::HashMap};
// use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, KeyboardEvent, MouseEvent, window};
use yew::prelude::*;
use yewdux::prelude::{Dispatch, use_store};

use crate::element_tree::{Attrs, EditorElement, ElementTree};
use crate::plugins::PluginTraits;

use super::PasteConverter;

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

        // let empty = "empty".to_string();
        // use_effect_with_deps(move |_my_text| {
        //TODO
        // let pasted_html = &fetch_pasted(&editor);
        // let json_pasted_html = JsFuture::from(pasted_html.json()?);
        // let parsed_pasted_html: FileTree = json.into_serde().unwrap();
        //     || {}
        // }, empty);

        // let handle_paste = Closure::wrap(Box::new(move |_e: KeyboardEvent| {
        //     _e.prevent_default();
        //     let p = _e.clipboard_data().unwrap();
        //     // let paste = (_e.clipboard_data().unwrap() || window().clipboard_data().unwrap()).get_data('text/html').unwrap();
        //     log_1(&format!("onpaste {:?}", _e).into());
        // }) as Box<dyn FnMut(_)>);
        //
        // &editor.add_event_listener_with_callback("paste", &handle_paste.as_ref().unchecked_ref());
        // &handle_paste.forget();
    }
}

impl PluginTraits for PasteConverter {}
