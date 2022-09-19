use yew::prelude::*;
use crate::components::{Menu, Avatar};
use web_sys::console::log_1;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {

    let position: UseStateHandle<String> = use_state(|| "".to_string());

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(
            format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
        );
    });
    let items: Vec<Html> = vec![
        html! {<><i class="fa-solid fa-user"></i>{"Profile info"}</>},
        html! {<><i class="fa-solid fa-eye"></i>{"Who can find me"}</>},
        html! {<><i class="fa-solid fa-gear"></i>{"Settings"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</>},
    ];

    html! { <>
    <Menu
    position={position.clone()}
     {items}
      />
    <span class="right_clickable main_avatar" {onmouseup}>
    <Avatar />
    </span>
    </>}
}
