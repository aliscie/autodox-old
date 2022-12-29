use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement};
use yew::prelude::*;
use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use shared::*;
use js_sys::{Uint8Array};
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
    let image: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 10, 8, 6, 0, 0, 0, 141, 50, 207, 189, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0, 68, 101, 88, 73, 102, 77, 77, 0, 42, 0, 0, 0, 8, 0, 1, 135, 105, 0, 4, 0, 0, 0, 1, 0, 0, 0, 26, 0, 0, 0, 0, 0, 3, 160, 1, 0, 3, 0, 0, 0, 1, 0, 1, 0, 0, 160, 2, 0, 4, 0, 0, 0, 1, 0, 0, 0, 10, 160, 3, 0, 4, 0, 0, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 59, 120, 184, 245, 0, 0, 0, 113, 73, 68, 65, 84, 24, 25, 133, 143, 203, 13, 128, 48, 12, 67, 147, 94, 97, 30, 24, 0, 198, 134, 1, 96, 30, 56, 151, 56, 212, 85, 68, 17, 88, 106, 243, 241, 235, 39, 42, 183, 114, 137, 12, 106, 73, 236, 105, 98, 227, 152, 6, 193, 42, 114, 40, 214, 126, 50, 52, 8, 74, 183, 108, 158, 159, 243, 40, 253, 186, 75, 122, 131, 64, 0, 160, 192, 168, 109, 241, 47, 244, 154, 152, 112, 237, 159, 252, 105, 64, 95, 48, 61, 12, 3, 61, 167, 244, 38, 33, 43, 148, 96, 3, 71, 8, 102, 4, 43, 140, 164, 168, 250, 23, 219, 242, 38, 84, 91, 18, 112, 63, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    let x = decode_image(image);
    // log!(x);

    // spawn_local(async move {
    //     // let response =  backend::get_profile().await;

    //     _image_link.set(Some("https://avatars.githubusercontent.com/u/58806996?v=4".to_string()));
    // });

    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
        })
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();


        spawn_local(async move {
            let buffer_bytes = encode_image(input.files().unwrap().get(0).unwrap()).await;
            log!(buffer_bytes);
            // let response =  backend::update_profile( Some(buffer_bytes), None).await;
        });
    });

    let items: Vec<Html> = vec![
        html! {<a><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><input onchange={on_upload} type="file" accept="image/*"/></a>},
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
            // let profile: JsValue = backend::get_profile().await;
            // log!(user_token);
        });
    });
    let auth = true; // serde_wasm_bindgen::from_value::<bool>(backend::is_logged().await).unwrap();
    let is_logged_ing = use_state(|| true);
    let _is_logged_ing = is_logged_ing.clone();

    if *is_logged_ing {
        return html! { <>

        <PopOverMenu {items} position = {position.clone()}/>
        <span class="right_clickable main_avatar" onclick={open_popover}>
        <img src={x}/>
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
