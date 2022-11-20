use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::backend;
use crate::components::{Avatar, Menu};
use crate::utils::DeviceInfo;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let device_info = Dispatch::<DeviceInfo>::new();

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a
        onclick={
            Callback::from(move |_| {
                    spawn_local(async move {backend::logout().await;})
            })
            }

        ><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];

    let onclick: Callback<MouseEvent> = device_info.reduce_mut_future_callback(|s| {
        Box::pin(async move {
            match serde_wasm_bindgen::from_value::<String>(backend::identify().await) {
                Ok(_x) => s.auth = true,
                Err(_) =>s.auth = false,
            }
        })
    });

    if device_info.get().auth {
        return html! { <>
        <Menu
        event={position.clone()}
         {items}
          />
        <span class="right_clickable main_avatar" {onmouseup}>
        <Avatar />
        </span>
        </>
        };
    } else {
        return html! {< span {onclick} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
    }
}
