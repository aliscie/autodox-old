use std::collections::HashMap;
use web_sys::Range;
use yew::Html;

#[derive(Clone, PartialEq)]
pub struct DropDownItem {
    pub(crate) value: Html,
    pub(crate) text: String,
    pub(crate) tag: Option<String>,
    pub(crate) attrs: HashMap<String, String>,
}

#[derive(Clone, PartialEq)]
pub struct DropDownItemEvent {
    pub(crate) value: Html,
    pub(crate) text: String,
    pub(crate) tag: Option<String>,
    pub(crate) attrs: HashMap<String, String>,
    pub(crate) range: Range,

}

pub type CommandItems = Vec<DropDownItem>;

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
