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
    pub toggle: UseStateHandle<String>,
}

#[function_component(GetTitleBar)]
pub fn get_titlebar(props: &Props) -> Html {
    let is_light_mode = use_state(|| false);
    let x = props.toggle.clone();

    let is_expanded = x.chars().count();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let current_directory = html! {<CurrDirectory/>};

    let handle_light_mod: Callback<MouseEvent> = {
        let is_light_mode = is_light_mode.clone();
        let doc = doc.clone();
        Callback::from(move |_e: MouseEvent| {
            let _ = doc
                .query_selector("html")
                .unwrap()
                .unwrap()
                .class_list()
                .toggle("light-mod");
            is_light_mode.set(!(*is_light_mode));
        })
    };

    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            let _ = &doc
                .query_selector(".editor_title")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:0px; width:100%");
            let _ = &doc
                .query_selector(".text_editor_container")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:0px; width:100%");
        } else {
            x.set("width:250px".to_string());
            let _ = &doc
                .query_selector(".editor_title")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
            let _ = &doc
                .query_selector(".text_editor_container")
                .unwrap()
                .unwrap()
                .set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
        }
    });

    let right_content: Html = html! {
       <>
               <Download/>
               <i
               onclick={handle_light_mod}
               class={format!("btn {}",if *is_light_mode {"fa-solid fa-moon"} else {"fa-solid fa-sun"})}
               ></i>

           <TitleAvatarComponent/>

           <PageOptions/>
       </>
    };
    let (device, _) = use_store::<DeviceInfo>();
    html! {
        <TitleBar
            style={(if !(device.is_web) {"padding-left: 75px; cursor: grab;"} else {""}).to_string()}
            title={current_directory}
            {right_content}
         >
            <li class="btn" onclick={toggle_asidebar}>
            {if is_expanded > 1{
                html!{<i class="fa-solid fa-x"></i>}
            } else{
                html!{<i class="fa-solid fa-bars"></i>}
            }}
            </li>
            <Markdown/>
        </TitleBar >
    }
}
