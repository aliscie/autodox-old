use crate::backend;
use crate::components::{Avatar, PopOverMenu};
use crate::pages::PagesRoute;
use crate::utils::{DeviceInfo, Image};
use gloo::timers::callback::Timeout;
use shared::schema::UserQuery;
use shared::*;
use std::sync::{Arc, Mutex};
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
    let dispatch_device_info = Dispatch::<DeviceInfo>::new();

    use_future_with_deps(
        move |_| async move {
            let auth = backend::is_logged().await.as_bool().unwrap();
            log!(auth);
            &dispatch_device_info.reduce_mut(|state| state.is_authed = auth);
            let register = backend::register("ali".to_string()).await;
            log!(register);
            let profile_res = backend::get_profile().await;
            // log!(&profile_res);
            let profile_obj: UserQuery = serde_wasm_bindgen::from_value(profile_res)
                .map_err(|e| String::from("serde error"))?;
            // log!(&profile_obj);
            &dispatch_device_info.reduce_mut(|state| state.profile = profile_obj);
            crate::hooks::init_files().await;
            return Ok(());
        },
        (),
    )
}

#[function_component]
pub fn TitleAvatarComponent() -> Html {
    use_profile();

    let (rc_device_info, _) = use_store::<DeviceInfo>();
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let navigator: yew_router::navigator::Navigator = use_navigator().unwrap();

    let on_popover: Callback<MouseEvent> = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            position.set(Some(e));
        })
    };

    let on_logout = Callback::from(move |e: MouseEvent| {
        spawn_local(async move {
            backend::logout().await;
            window().unwrap().location().reload().unwrap();
        })
    });

    let on_upload: Callback<Event> = Callback::from(move |_e: Event| {
        let input: HtmlInputElement = _e.target_unchecked_into();
        let mut profile_arc = Arc::new(Mutex::new(UserQuery {
            username: Some("account1".to_string()),
            image: None,
        }));
        let profile_arc_clone = Arc::clone(&profile_arc);

        spawn_local(async move {
            let file = input.files().unwrap().get(0).unwrap();
            let image = Image::new(file).await;
            let profile_obj = &mut *profile_arc_clone.lock().unwrap();
            profile_obj.image = Some(image.data.clone());
            // log!(&profile_obj);
            let profile_json = serde_json::json!(profile_obj);
            let response = backend::update_profile(profile_json.to_string()).await;
            log!(&response);
        });

        Timeout::new(1000, move || {
            // let profile_obj = profile_arc.lock().unwrap().clone();
        })
        .forget();
    });

    let items: Vec<Html> = vec![
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Profile)}}}><i class="fa-solid fa-user"></i>{"Profile info"}</a>},
        html! {<a><input onchange={on_upload} type="file" accept="image/*"/></a>},
        html! {<a><i class="fa-solid fa-eye"></i>{"Who can find me"}</a>},
        html! {<a onclick = {let navigator=navigator.clone();{move |_| {navigator.push(&PagesRoute::Settings)}}}><i class="fa-solid fa-gear"></i>{"Settings"}</a>},
        html! {<a onmousedown={on_logout}><i class="fa-solid fa-right-from-bracket"></i>{"logout"}</a>},
    ];

    let on_login = Callback::from(move |_| {
        spawn_local(async move {
            let user_token = backend::identify().await;
            // window().unwrap().location().reload().unwrap();
        });
    });

    if rc_device_info.is_authed {
        return html! { <>
            <PopOverMenu {items} position = {position.clone()}/>
            <span class="right_clickable main_avatar" onclick={on_popover}>
                <Avatar src={Image::get_opt_link(rc_device_info.profile.image.clone())}/>
            </span>
        </>
        };
    };
    return html! {<span onclick={on_login} class = "btn"> <i class = "fa-solid fa-right-to-bracket"></ i>{"login"}</ span>};
}
