use crate::backend;
use crate::backend::QueryUser;
use crate::components::{Avatar, PopOverMenu};
use crate::pages::PagesRoute;
use crate::utils::{DeviceInfo, Image};
use shared::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement, Navigator};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yew_router::Routable;
use yewdux::functional::use_store;

#[function_component]
pub fn TitleAvatarComponent() -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    let _dispatch = dispatch.clone();
    let image_opt_vec: UseStateHandle<Vec<u8>> = use_state(|| {
        vec![
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 10, 0, 0, 0, 10,
            8, 6, 0, 0, 0, 141, 50, 207, 189, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0,
            0, 0, 68, 101, 88, 73, 102, 77, 77, 0, 42, 0, 0, 0, 8, 0, 1, 135, 105, 0, 4, 0, 0, 0,
            1, 0, 0, 0, 26, 0, 0, 0, 0, 0, 3, 160, 1, 0, 3, 0, 0, 0, 1, 0, 1, 0, 0, 160, 2, 0, 4,
            0, 0, 0, 1, 0, 0, 0, 10, 160, 3, 0, 4, 0, 0, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 59, 120,
            184, 245, 0, 0, 0, 113, 73, 68, 65, 84, 24, 25, 133, 143, 203, 13, 128, 48, 12, 67,
            147, 94, 97, 30, 24, 0, 198, 134, 1, 96, 30, 56, 151, 56, 212, 85, 68, 17, 88, 106,
            243, 241, 235, 39, 42, 183, 114, 137, 12, 106, 73, 236, 105, 98, 227, 152, 6, 193, 42,
            114, 40, 214, 126, 50, 52, 8, 74, 183, 108, 158, 159, 243, 40, 253, 186, 75, 122, 131,
            64, 0, 160, 192, 168, 109, 241, 47, 244, 154, 152, 112, 237, 159, 252, 105, 64, 95, 48,
            61, 12, 3, 61, 167, 244, 38, 33, 43, 148, 96, 3, 71, 8, 102, 4, 43, 140, 164, 168, 250,
            23, 219, 242, 38, 84, 91, 18, 112, 63, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ]
    });
    let _image_opt_vec = image_opt_vec.clone();
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let navigator: yew_router::navigator::Navigator = use_navigator().unwrap();
    let auth = true; // serde_wasm_bindgen::from_value::<bool>(backend::is_logged().await).unwrap();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let auth = backend::is_logged().await.as_bool().unwrap();
                log!("before login");
                &_dispatch.reduce_mut(|state| state.is_authed = auth);
                let register = backend::register("ali".to_string()).await;
                log!(register);
                let get_profile = backend::get_profile().await;
                log!(&get_profile);
                // let s = js_sys::JSON::stringify(&get_profile)
                //     .map(String::from)
                //     .unwrap();
                // log!(serde_json::from_str::<QueryUser>(&s));
                log!(serde_wasm_bindgen::from_value::<QueryUser>(get_profile));
                // log!(Uint8Array::new(&get_profile).to_vec());
                // _image_opt_vec.set(get_profile)
            });
        },
        (),
    );

    let x = Image::to_link(&*_image_opt_vec);

    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
            window().unwrap().location().reload().unwrap();
        })
    });

    let open_popover: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();

        spawn_local(async move {
            let file = input.files().unwrap().get(0).unwrap();
            let image = Image::new(file).await;
            log!(&image.data);
            let response = backend::update_profile("account1".to_string(), image.data).await;
            log!(response);
        });
    });

    let on_login = Callback::from(move |_| {
        spawn_local(async move {
            // let x = invoke_async("open_new_window".to_string()).await;
            // TODO
            //     if devices.is_web {
            //         window.open_new_window(),
            //     } else {
            //         let x = invoke_async("open_new_window".to_string()).await;
            //     }
            let user_token = backend::identify().await;
            // window().unwrap().location().reload().unwrap();
        });
    });

    let items: Vec<Html> = vec![
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Profile)}}} ><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><input onchange={on_upload} type="file" accept="image/*"/></a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Settings)}}} ><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={logout} ><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];

    if device.is_authed {
        return html! {
            <>
                <PopOverMenu {items} position = {position.clone()}/>
                <span class="right_clickable main_avatar" onclick={open_popover}>
                <Avatar src={x} />
                </span>
            </>
        };
    };
    return html! {< span onclick={on_login} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
}
