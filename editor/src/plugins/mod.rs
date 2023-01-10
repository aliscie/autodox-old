use web_sys::Element;

pub use editor_toolbar_plugin::*;
pub use inset_component::*;
pub use paste_converter::PasteConverter;

mod paste_converter;
mod inset_component;

mod drag_and_drop;
mod mention;
mod editor_toolbar_plugin;

// use web_sys::{Element, MouseEvent, window, Document};
// use wasm_bindgen::UnwrapThrowExt;

pub trait EditorPlugin {
    // fn remove_menu<'a>(curr: &'a Element, doc: &'a Document);
    // fn make_menu<'a>( curr: &'a Element, doc: &'a Document);
}

pub trait PluginTraits {}

impl<T> EditorPlugin for T
    where
        T: PluginTraits,
{
    // fn remove_menu<'a>(curr: &'a Element, doc: &'a Document) {}
    // fn make_menu<'a>( curr: &'a Element, doc: &'a Document) {
    // }
}
