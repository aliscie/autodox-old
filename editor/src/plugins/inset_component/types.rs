use std::collections::HashMap;
use web_sys::Range;
use yew::Html;

#[derive(Clone, PartialEq, Debug)]
pub struct DropDownItem {
    pub value: String,
    pub text: String,
    pub tag: Option<String>,
    pub attrs: HashMap<String, String>,
}

pub type CommandItems = Vec<DropDownItem>;

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
