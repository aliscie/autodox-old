use yew::prelude::*;
use crate::components::{Menu, Avatar};
use web_sys::console::log_1;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let display: UseStateHandle<bool> = use_state(|| false);

    let _display = display.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _display.set(true);

    });
    let items: Vec<Html> = vec![
        html! {<><i class="gg-eye-alt"></i>{"Who can find me"}</>},
        html! {<><i class="gg-log-off"/>{"logout"}</>},
    ];

    html! { <>
    <Menu  {items} {display}/>
    <span class="right_clickable main_avatar" {onmouseup}>
    <Avatar />
    </span>
    </>}
}
