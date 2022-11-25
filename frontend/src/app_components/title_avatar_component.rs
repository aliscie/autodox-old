use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;


use crate::backend;
use crate::components::{Avatar, Menu};

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);

    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move { backend::logout().await; })
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-user"></i>{"Profile info"}</a>},
        html! {<a><i class=" fa-eye"></i>{"Who can find me"}</a>},
        html! {<a><i class=" fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={logout} ><i class="fa-right-from-bracket"></i>{"logout"}</a>},
    ];

    let onclick = Callback::from(move |_e: MouseEvent| {
        spawn_local(async move {
            // let x = invoke_async("open_new_window".to_string()).await;
            // TODO
            //     if IS_WEB {
            //         window.open_new_window(),
            //     } else {
            //         let x = invoke_async("open_new_window".to_string()).await;
            //     }
            let user_token = backend::identify().await;
            // log!(user_token);
        });
    });

    let is_logged_ing = use_state(|| false);
    let _is_logged_ing = is_logged_ing.clone();
    spawn_local(async move {
        let auth = serde_wasm_bindgen::from_value::<bool>(backend::is_logged().await).unwrap();
        _is_logged_ing.set(auth);
    });

    if *is_logged_ing {
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
