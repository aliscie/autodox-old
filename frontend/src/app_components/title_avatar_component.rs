use yew::prelude::*;
use crate::components::{Menu, Avatar};
use web_sys::console::log_1;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let display: UseStateHandle<bool> = use_state(|| false);
    let position: UseStateHandle<String> = use_state(|| "".to_string());

    let _display = display.clone();
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _display.set(true);
        _position.set(
            format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
        );
    });
    let items: Vec<Html> = vec![
        html! {<><i class="fa-solid fa-eye"></i>{"Who can find me"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</>},
    ];

    html! { <>
    <Menu position={position.clone()}  {items} {display}/>
    <span class="right_clickable main_avatar" {onmouseup}>
    <Avatar />
    </span>
    </>}
}
