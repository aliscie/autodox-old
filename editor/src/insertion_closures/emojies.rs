use std::collections::HashMap;
use yew::html;
use shared::log;
use crate::plugins::{CommandItems, DropDownItem};
use emojis;

pub fn emojies() -> CommandItems {
    let mut emojis = CommandItems::new();

    for i in emojis::iter() {
        emojis.push(DropDownItem {
            value: html! { i.emoji },
            text: i.name.to_string(),
            tag: None,
            attrs: HashMap::new(),
        });
    }
    emojis
}

