use rand::seq::SliceRandom;
use yew::prelude::*;
use yew::{function_component, html, Html};

use shared::*;

use crate::components::ContextMenu;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component(ButtonsGroup)]
pub fn buttons_group(props: &Props) -> Html {
    let click_on = use_state_eq(|| false);
    let category = use_state_eq(|| "home".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let style = use_state_eq(|| "display:none".to_string());

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let _click_on = click_on.clone();
    let _style = style.clone();
    let onmousedown: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _click_on.set(true);
        _style.set("display:inline-block;animation-name: btnEntrance; transition:0.2s".to_string())
    });

    // let _click_on = click_on.clone();
    // let onmousedown_others: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
    //     _click_on.set(false);
    // });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-home"></i>{"Home"}</a>},
        html! {<a><i class="fa-solid fa-trash"></i>{"Trash"}</a>},
        html! {<a>{"School"}</a>},
        html! {
        <a {onmousedown}>
        <span style={format!("{}",&*style.clone())}><input  placeholder="rename.."/></span>
        <i style="display:inline-block" class="fa-solid fa-plus"></i>{"add category"}
        </a>
        },
    ];

    html! {
    <>
        <span {onmouseup} class="ptn" style="width:100%;">{format!("{}",*category.clone())}</span>
    </>
    }
}
