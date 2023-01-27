use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn mentions() -> CommandItems {
    vec![
        DropDownItem {
            value: html! {<a>{"Ali"}</a>},
            text: "Ali".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Aman"}</a>},
            text: "Aman".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {<a>{"Young"}</a>},
            text: "Young".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}