use crate::utils::alert;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::router::*;
use yew_router::prelude::*;


#[cfg(not(feature = "web"))]
use shared::invoke;

use crate::components::{TitleBar, TitleBarButton, CurrDirectory, Avatar};
use crate::app_components::{TitleAvatarComponent};

use web_sys::{window, Document, Element, MouseEvent};


pub fn get_titlebar(x: UseStateHandle<String>) -> Html {
    let is_expanded = x.clone().chars().count();
    let doc = window().unwrap_throw().document().unwrap_throw();
    let current_directory = html! {<CurrDirectory/>};

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

    let current_directory = html! {<CurrDirectory/>};
    let mut is_web = true;
    #[cfg(not(feature = "web"))] {
        is_web = false;
    }
    return html! {
        <TitleBar
        style={format!("{}",if is_web==false {"margin-left: 75px"} else {""})}
         title={current_directory}>
            <div class="buttons">
            <button style="width: 25px; font-size:1px;" onclick={toggle_asidebar} button_type="toggle">
            {if is_expanded > 1{
                html!{<i class="gg-close"></i>}
            } else{
                html!{<i class="gg-menu"></i>}
            }}
            </button>
            </div>

            <div style="position:fixed;right:8%;">
                {if is_web { html!{<button><i class="gg-software-download"></i>{"Download"}</button>} } else {html!{""}}}
                <TitleAvatarComponent/>
            </div>

        </TitleBar >
    };
}

