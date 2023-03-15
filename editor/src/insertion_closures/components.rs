use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn components() -> CommandItems {
    vec![
        DropDownItem {
            value: "table".to_string(),
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
            value: "link".to_string(),
            text: "link".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "list".to_string(),
            text: "list".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "code".to_string(),
            text: "code".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "quote".to_string(),
            text: "quote".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: "title".to_string(),
            text: "title".to_string(),
            tag: Some("h1".to_string()),
            attrs: HashMap::new(),
        },
    ]
}