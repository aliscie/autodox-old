use wasm_bindgen::prelude::*;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use crate::utils::DeviceInfo;
use yewdux::prelude::*;

use crate::specific_components::{Download, Markdown, PageOptions, TitleAvatarComponent};
use crate::components::{CurrDirectory, TitleBar};
use crate::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    // pub toggle: UseStateHandle<String>,
}

#[function_component(GetTitleBar)]
pub fn get_titlebar(props: &Props) -> Html {
    let (device, dispatch) = use_store::<DeviceInfo>();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let current_directory = html! {<CurrDirectory/>};

    let _device = device.clone();
    let handle_light_mod: Callback<MouseEvent> = {
        let dispatch = dispatch.clone();
        let doc = doc.clone();
        Callback::from(move |_e: MouseEvent| {
            let _ = doc
                .query_selector("html")
                .unwrap()
                .unwrap()
                .class_list()
                .toggle("light-mod");
            &dispatch.reduce_mut(|state| state.is_light_mode = !_device.is_light_mode);
        })
    };
    let _dispatch = dispatch.clone();
    let _device = device.clone();
    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if _device.is_aside {
            &_dispatch.reduce_mut(|state| state.is_aside = false);
        } else {
            &_dispatch.reduce_mut(|state| state.is_aside = true);
        }
    });
    let _device = device.clone();
    let right_content: Html = html! {
       <>
               <Download/>
               <i
               onclick={handle_light_mod}
               class={format!("btn {}",if _device.is_light_mode {"fa-solid fa-moon"} else {"fa-solid fa-sun"})}
               ></i>

           <TitleAvatarComponent/>

           <PageOptions/>
       </>
    };

    html! {
        <TitleBar
            style={(if !(device.is_web) {"padding-left: 75px; cursor: grab;"} else {""}).to_string()}
            title={current_directory}
            {right_content}
         >
            <li class="btn" onclick={toggle_asidebar}>
            {if device.is_aside {
                html!{<i class="fa-solid fa-x"></i>}
            } else{
                html!{<i class="fa-solid fa-bars"></i>}
            }}
            </li>
            <Markdown/>
        </TitleBar >
    }
}
