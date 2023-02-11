use crate::components::{CurDirectory, TitleBar};
use crate::specific_components::{SaveButton, Download, Markdown, PageOptions, TitleAvatarComponent};
use crate::utils::DeviceInfo;
use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(GetTitleBar)]
pub fn get_title_bar() -> Html {
    let (rc_device_info, dispatch_device_info) = use_store::<DeviceInfo>();
    let document = window().unwrap_throw().document().unwrap_throw();

    let _rc_device_info = rc_device_info.clone();
    let _dispatch_device_info = dispatch_device_info.clone();
    let on_light_mode: Callback<MouseEvent> = {
        Callback::from(move |_e: MouseEvent| {
            let _ = document
                .query_selector("html")
                .unwrap()
                .unwrap()
                .class_list()
                .toggle("light-mode");
            let _ = &_dispatch_device_info
                .reduce_mut(|state| state.is_light_mode = !_rc_device_info.is_light_mode);
        })
    };

    let _dispatch_device_info = dispatch_device_info.clone();
    let _rc_device_info = rc_device_info.clone();
    let on_aside_bar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _rc_device_info.is_aside {
            let _ = &_dispatch_device_info.reduce_mut(|state| state.is_aside = false);
        } else {
            let _ = &_dispatch_device_info.reduce_mut(|state| state.is_aside = true);
        }
    });

    let right_content: Html = html! {
        <>
            <SaveButton/>
            <Download/>
            <i
                onclick={on_light_mode}
                class={format!("btn {}",if rc_device_info.is_light_mode {"fa-solid fa-moon"} else {"fa-solid fa-sun"})}
            ></i>
            <Suspense fallback = {html! {<div class="titlebar loader"/>}}>
                <TitleAvatarComponent/>
            </Suspense>
            <PageOptions/>
        </>
    };

    html! {
        <TitleBar
            style={(if !(rc_device_info.is_web) {"padding-left: 75px; cursor: grab;"} else {""}).to_string()}
            title={html! {<CurDirectory/>}}
            {right_content}
         >
            <li class="btn" onclick={on_aside_bar}>
                {if rc_device_info.is_aside {
                    html!{<i class="fa-solid fa-x"></i>}
                } else{
                    html!{<i class="fa-solid fa-bars"></i>}
                }}
            </li>
            <Markdown/>
        </TitleBar >
    }
}
