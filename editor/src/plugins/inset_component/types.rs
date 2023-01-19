use std::collections::HashMap;
use yew::Html;

#[derive(Clone, PartialEq)]
pub struct DropDownItem {
    pub(crate) value: Html,
    pub(crate) text: String,
    pub(crate) tag: Option<String>,
    pub(crate) attrs: HashMap<String, String>,
}

pub type CommandItems = Vec<DropDownItem>;

#[derive(Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
