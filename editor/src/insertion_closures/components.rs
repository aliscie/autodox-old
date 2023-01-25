use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn components() -> CommandItems {
    vec![
        DropDownItem {
            value: html! {<a>{"Table"}</a>},
            text: "table".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Image"}</a>},
            text: "image".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Video"}</a>},
            text: "video".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Audio"}</a>},
            text: "audio".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Link"}</a>},
            text: "link".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"List"}</a>},
            text: "list".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Code"}</a>},
            text: "code".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Quote"}</a>},
            text: "quote".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}