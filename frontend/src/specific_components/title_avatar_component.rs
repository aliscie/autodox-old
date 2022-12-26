use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, HtmlInputElement};
use yew::prelude::*;
use gloo::file::File;
use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use shared::*;
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::JsValue;
 use web_sys::Url;
#[function_component(TitleAvatarComponent)]
pub fn title_avatar_component() -> Html {
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let open_popover: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
        })
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();
        // let mut result = Vec::new();
        // if let Some(files) = input.files() {
        //  let files = js_sys::try_iter(&files)
        //         .unwrap()
        //         .unwrap()
        //         .map(|v| web_sys::File::from(v.unwrap()))
        //         .map(File::from);
        //     result.extend(files);
        // }
        spawn_local(async move {
            let buffer = input.files().unwrap().get(0).unwrap().array_buffer().value_of();
            let bytes: Vec<u8> = Uint8Array::new(&buffer).to_vec();
            // TODO
            //  let image_url  = Uint8Array::new(&bytes);
            //  let x = Url.create_object_url_with_u8_array(Uint8Array::new(&bytes)).unwrap();
            //  let bytes: Vec<u8> = Uint8Array::new(&buffer).try_into().unwrap();
            //  let image_url = Blob::new_with_buffer_source_sequence_and_options(&vec![bytes], "image/png").unwrap();
            // log!(bytes);
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
        <Avatar />
        </span>
        </>
        };
    } else {
        return html! {< span {onclick} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
    }
}
