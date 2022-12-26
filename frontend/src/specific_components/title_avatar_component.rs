use wasm_bindgen_futures::spawn_local;
use web_sys::{ HtmlInputElement};
use yew::prelude::*;
use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use shared::*;
use js_sys::{ Uint8Array};
use wasm_bindgen::JsValue;
use web_sys::Url;
use crate::utils::{decode_image, encode_image};

#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let image_link: UseStateHandle<Option<String>> = use_state(|| None);
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();


    let open_popover: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let _image_link = image_link.clone();
    spawn_local(async move {
        // let response =  backend::get_profile().await;
        // decode_image(response);
        _image_link.set(Some("https://avatars.githubusercontent.com/u/58806996?v=4".to_string()));
    });
    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
        })
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();

        let buffer_bytes = encode_image(input.files().unwrap().get(0).unwrap());
        spawn_local(async move {
            // log!(buffer_bytes);
            // let response =  backend::update_profile( Some(buffer_bytes), None).await;
        });
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><input onchange={on_upload} type="file" id="img" name="img" accept="image/*"/></a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={logout} ><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];

    let onclick = Callback::from(move |_e: MouseEvent| {
        spawn_local(async move {
            // let x = invoke_async("open_new_window".to_string()).await;
            // TODO
            //     if devices.is_web {
            //         window.open_new_window(),
            //     } else {
            //         let x = invoke_async("open_new_window".to_string()).await;
            //     }
            let user_token = backend::identify().await;
            let profile: JsValue = backend::get_profile().await;
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
        <PopOverMenu {items} position = {position.clone()}/>
        <span class="right_clickable main_avatar" onclick={open_popover}>
        <Avatar
            // src={image_link.clone()} TODO fix this
            />
        </span>
        </>
        };
    } else {
        return html! {< span {onclick} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
    }
}
