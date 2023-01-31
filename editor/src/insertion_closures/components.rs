use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn components() -> CommandItems {
    vec![
        DropDownItem {
            value: "Table".to_string(),
            text: "table".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "image".parse().unwrap(),
            text: "image".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "video".parse().unwrap(),
            text: "video".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Table".to_string(),
            text: "audio".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Table".to_string(),
            text: "link".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Table".to_string(),
            text: "list".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Table".to_string(),
            text: "code".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "Table".to_string(),
            text: "quote".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}