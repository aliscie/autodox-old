use std::collections::HashMap;
use shared::Tree;
use serde::{Serialize, Deserialize};


pub struct ElementTree{
    elements : Tree<u64, EditorElement>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq , PartialEq, Default)]
pub struct EditorElement{
    text : String,
    attrs : HashMap<Attrs, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq , PartialEq, Default, Hash)]
pub enum Attrs{
    #[default] Style,
}
