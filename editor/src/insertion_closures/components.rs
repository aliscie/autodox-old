use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn components() -> CommandItems {
    vec![
        DropDownItem {
            value: html! {<a>{"Table"}</a>},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"quote"}</a>},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"heading"}</a>},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}