use std::collections::HashMap;
use yew::{AttrValue, Children, function_component, Html};
use yew::virtual_dom::VTag;
use indexmap::IndexMap;
use shared::id::Id;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConstractorProps {
    pub tag: Option<String>,
    pub attrs: HashMap<String, String>,
    pub text: String,
    pub id: Id,
    #[prop_or_default]
    pub children: Children,
}


#[function_component]
pub fn ConstructElement(props: &ConstractorProps) -> Html {

    let mut tag = props.tag.clone();
    let tag = format!("{}", tag.unwrap_or(String::from("div")));
    let mut tag = VTag::new(tag);
    let mut attrs: IndexMap<AttrValue, AttrValue> = props
        .attrs
        .clone()
        .into_iter()
        .map(|(key, value)| -> (AttrValue, AttrValue) { (key.into(), value.into()) })
        .collect();
    // setting id
    attrs.insert("id".into(), props.id.to_string().into());
    //adding the text
    tag.add_child(html! { {props.text.clone()}});
    // adding children
    tag.add_children(props.children.clone());
    // setting attributes
    tag.set_attributes(attrs);
    tag.into()
}
