use crate::utils::alert;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::router::*;
use yew_router::prelude::*;


#[cfg(not(feature = "web"))]
use shared::invoke;

use crate::components::{TitleBar, TitleBarButton, CurrDirectory};

use web_sys::{window, Document, Element, MouseEvent};


pub fn get_titlebar(x: UseStateHandle<String>) -> Html {
    let is_expanded = x.clone().chars().count();
    let doc = window().unwrap_throw().document().unwrap_throw();


    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            doc.query_selector(".text_editor_container").unwrap().unwrap().set_attribute("style", "margin-left:0px; width:100%");
        } else {
            x.set("width:250px".to_string());
            doc.query_selector(".text_editor_container").unwrap().unwrap().set_attribute("style", "margin-left:250px; margin-right:2%; width:80%");
        }
    });
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "web"))]{
        let toggle_maximize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
            let _ = invoke::<String>("maximize_window".into(), None).map_err(|e| alert(&e));
        });
        let toggle_minimize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
            let _ = invoke::<String>("minimize_window".into(), None).map_err(|e| alert(&e));
        });
        let close_window: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
            let _ = invoke::<String>("close_window".into(), None).map_err(|e| alert(&e));
        });
            let current_directory = html!{<CurrDirectory/>};
            return html!{
                <TitleBar title={current_directory}>
                <div style="margin-left:60px" >
                // <TitleBarButton onclick = {close_window} button_type="close">{""}
                // </TitleBarButton>
                // <TitleBarButton onclick = {toggle_minimize} button_type="minimize">{""}
                // </TitleBarButton>
                // <TitleBarButton onclick={toggle_maximize}button_type="zoom">{""}</TitleBarButton>
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">
                {""}
                </TitleBarButton>
                // <TitleBarButton button_type="next_back">{"←"}</TitleBarButton>
                // <TitleBarButton button_type="next_back">{"→"}</TitleBarButton>
                // <TitleBarButton button_type="share">{"⤿"}</TitleBarButton>
                // <TitleBarButton button_type="star">{"★"}</TitleBarButton>
                // <TitleBarButton button_type="toggle">{"👨‍💼"}</TitleBarButton>
                // <TitleBarButton button_type="toggle">{"💬"}</TitleBarButton>
                // <TitleBarButton button_type="toggle">{"..."}</TitleBarButton>
                </div>
                </TitleBar>

            }
        }
        else {
            return html!{
                <TitleBar title="filename/filename4">
                <div>
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">
                {if is_expanded > 1{"X"} else{"="}}
                </TitleBarButton>
                </div>
                </TitleBar>
            }
        }
    }
}

