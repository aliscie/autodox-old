use yew::prelude::*;
use crate::components::{Menu, Avatar};
use crate::*;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let items: Vec<Html> = vec![
        html! {<><i class="fa-solid fa-user"></i>{"Profile info"}</>},
        html! {<><i class="fa-solid fa-eye"></i>{"Who can find me"}</>},
        html! {<><i class="fa-solid fa-gear"></i>{"Settings"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</>},
    ];
    if *IS_LOGEDIN {
        html! { <>
                <Menu
                event={position.clone()}
                 {items}
                  />
                <span class="right_clickable main_avatar" {onmouseup}>
                <Avatar />
                </span>
                </>
                }
    } else {
        html! {<span class="btn" ><i class="fa-solid fa-right-to-bracket"></i>{"login"}</span>}
    }
}
