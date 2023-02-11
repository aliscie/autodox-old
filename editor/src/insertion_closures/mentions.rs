use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn mentions() -> CommandItems {
    vec![
        DropDownItem {
            value: "Ali".parse().unwrap(),
            text: "Ali".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Ali".parse().unwrap(),
            text: "Aman".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Ali".parse().unwrap(),
            text: "Young".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}