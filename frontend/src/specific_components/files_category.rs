use crate::components::PopOverMenu;
use rand::seq::SliceRandom;
use shared::*;
use yew::prelude::*;
use yew::{function_component, html, Html};

#[function_component]
pub fn FilesCategory() -> Html {
    let click_on = use_state_eq(|| false);
    let category = use_state_eq(|| "home".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let style = use_state_eq(|| "display:none".to_string());

    let _position = position.clone();
    let on_mouse_up: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let _click_on = click_on.clone();
    let _style = style.clone();
    let on_mouse_down: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _click_on.set(true);
        _style.set("display:inline-block;animation-name: btnEntrance; transition:0.2s".to_string())
    });

    let items: Vec<Html> = vec![
        html! {<option>{"Home"}</option>},
        html! {<option>{"Trash"}</option>},
        html! {<option>{"School"}</option>},
        html! {<option>{"Add a category +"}</option>},
    ];

    html! {
        <select style="width:100%"  onmouseup={on_mouse_up} >
            {items}
        </select>
    }
}
