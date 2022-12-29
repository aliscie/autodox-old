use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::functional::use_store;
use shared::log;

use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use crate::utils::DeviceInfo;

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();

    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();

    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
        });
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={logout} ><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];
    let _dispatch = dispatch.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let auth = backend::is_logged().await.as_bool().unwrap();
                _dispatch.reduce_mut(|state| state.is_authed = auth);

                let test_wasm = backend::test_wasm().await;
                log!( test_wasm);
                //
                // let test_ic = backend::test_ic().await;
                // log!( test_ic);
            });
        },
        (),
    );

    let login_event = Callback::from(move |_e: MouseEvent| {
        spawn_local(async move {
            // let x = invoke_async("open_new_window".to_string()).await;
            // TODO
            //     if devices.is_web {
            //         window.open_new_window(),
            //     } else {
            //         let x = invoke_async("open_new_window".to_string()).await;
            //     }
            let user_token = backend::identify().await;
        });
    });


    if device.is_authed {
        return html! { <>
        <PopOverMenu items = {items} position = {position.clone()}/>
        <span class="right_clickable main_avatar" {onmouseup}>
        <Avatar />
        </span>
        </>
        };
    } else {
        return html! {< span onclick={login_event} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
    }
}
