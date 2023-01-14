use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use crate::pages::PagesRoute;
use crate::utils::{DeviceInfo, Image};
use shared::schema::UserQuery;
use shared::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew::suspense::SuspensionResult;
use yew::suspense::UseFutureHandle;
use yew_router::prelude::use_navigator;
use yewdux::functional::use_store;
use yewdux::prelude::Dispatch;

#[hook]
fn use_profile() -> SuspensionResult<UseFutureHandle<Result<(), String>>> {
    let dispatch = Dispatch::<DeviceInfo>::new();

    use_future_with_deps(
        move |_| async move {
            let auth = backend::is_logged().await.as_bool().unwrap();
            log!(auth);
            &dispatch.reduce_mut(|state| state.is_authed = auth);
            let register = backend::register("ali".to_string()).await;
            log!(register);
            let profile_res = backend::get_profile().await;
            log!(&profile_res);
            let profile_obj: UserQuery = serde_wasm_bindgen::from_value(profile_res)
                .map_err(|e| String::from("serde error"))?;
            &dispatch.reduce_mut(|state| state.profile = profile_obj);
            return Ok(());
        },
        (),
    )
}

#[function_component]
pub fn TitleAvatarComponent() -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    use_profile();

    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let open_popover: Callback<MouseEvent> = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
        })
    };

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
            let response = backend::update_profile("account1".to_string(), image.data).await;
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
                <Avatar src={Image::to_link(device.profile.image.clone())} />
            </span>
        </>
        };
    };
    return html! {< span onclick={on_login} class = "btn" > < i class = "fa-solid fa-right-to-bracket" >< / i >{"login"}< / span >};
}
