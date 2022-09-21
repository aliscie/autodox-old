use crate::utils::alert;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::router::*;
use yew_router::prelude::*;

use web_sys::console::log_1;

use shared::invoke;

use crate::components::{TitleBar, CurrDirectory, Avatar};
use crate::app_components::{TitleAvatarComponent, PageOptions, Download};

use web_sys::{window, Document, Element, MouseEvent};
use crate::*;

pub fn get_titlebar(x: UseStateHandle<String>) -> Html {
    let light_mod = use_state(|| false);


    let is_expanded = x.clone().chars().count();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let current_directory = html! {<CurrDirectory/>};


    let _light_mod = light_mod.clone();
    let _doc = doc.clone();
    let handle_light_mod: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        _doc.query_selector("#app").unwrap().unwrap().class_list().toggle("night-mod");
        _light_mod.set(!(*_light_mod));
    });

    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            doc.query_selector(".editor_title").unwrap().unwrap().set_attribute("style", "margin-left:0px; width:100%");
            doc.query_selector(".text_editor_container").unwrap().unwrap().set_attribute("style", "margin-left:0px; width:100%");
        } else {
            x.set("width:250px".to_string());
            doc.query_selector(".editor_title").unwrap().unwrap().set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
            doc.query_selector(".text_editor_container").unwrap().unwrap().set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
        }
    });


    let right_content: Html = html! {
            <>
                    <Download/>
                    <i
                    onclick={handle_light_mod}
                    class={format!("btn {}",if (*light_mod).clone() {"fa-solid fa-moon"} else {"fa-solid fa-sun"})}
                    ></i>

                <TitleAvatarComponent/>

                <PageOptions/>
            </>
         };
    return html! {
        <TitleBar
            style={format!("{}",if *IS_WEB==false {"padding-left: 75px; cursor: grab;"} else {""})}
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
        </TitleBar >
    };
}

