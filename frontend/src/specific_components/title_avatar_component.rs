use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::functional::use_store;

use shared::*;
use shared::schema::QueryUser;

use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use crate::pages::PagesRoute;
use crate::utils::{DeviceInfo, Image};

#[function_component]
pub fn TitleAvatarComponent() -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    let _dispatch = dispatch.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let auth = backend::is_logged().await.as_bool().unwrap();
                log!("before login");
                &_dispatch.reduce_mut(|state| state.is_authed = auth);
                let register = backend::register("ali".to_string()).await;
                log!(register);

                let get_profile: QueryUser = serde_wasm_bindgen::from_value(backend::get_profile().await).unwrap();
                // todo why get_profile.image always none?
                &_dispatch.reduce_mut(|state| state.profile = get_profile);
            });
        },
        (),
    );

    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let _position = position.clone();
    let open_popover: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(Some(_e));
    });
    let mut image: Option<String> = None;
    // TODO fix this
    //  if !device.profile.image.is_none() {
    //      image = Image::to_link(device.profile.image.unwrap());
    //  }


    let logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
            window().unwrap().location().reload().unwrap();
        })
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();

        spawn_local(async move {
            let file = input.files().unwrap().get(0).unwrap();
            let image = Image::new(file).await;
            log!(&image.data);
            let response = backend::update_profile(image.data).await;
            log!(response);
        });
    });
    let navigator: yew_router::navigator::Navigator = use_navigator().unwrap();


    let items: Vec<Html> = vec![
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Profile)}}} ><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><input onchange={on_upload} type="file" accept="image/*"/></a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Settings)}}} ><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={logout} ><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];

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

    if device.is_authed {
        return html! { <>

        <PopOverMenu {items} position = {position.clone()}/>
        <span class="right_clickable main_avatar" onclick={open_popover}>
        <Avatar
            // src={image}
            />
        </span>
        </>
        };
    };
    return html! {< span onclick={on_login} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
}
