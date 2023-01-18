use yew::Html;

#[derive(Clone, PartialEq)]
pub struct DropDownItem {
    pub(crate) value: Html,
    pub(crate) insertion: Html,
}

pub type CommandItems = Vec<DropDownItem>;

#[derive(Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
