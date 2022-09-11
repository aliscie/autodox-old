use yew::prelude::*;
use crate::components::{Menu, Avatar};

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let display: UseStateHandle<bool> = use_state(|| false);

    let _display = display.clone();
    let onmousedown: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _e.which() == 3{
        _display.set(true);
        }

    });
    let items: Vec<Html> = vec![
        html! {<><i class="gg-eye-alt"></i>{"Who can find me"}</>},
        html! {<><i class="gg-log-off"/>{"logout"}</>},
    ];

    html! { <>

    <span class="right_clickable" {onmousedown}>
    <Avatar />
    </span>

    <Menu {items} {display}/>
    </>}
}
