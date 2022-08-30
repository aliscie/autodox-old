use crate::utils::alert;
use wasm_bindgen::prelude::*;
use yew::prelude::*;


#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use crate::components::{TitleBar, TitleBarButton};

use web_sys::{window, Document, Element, MouseEvent};


pub fn get_titlebar(article_position: UseStateHandle<String>, x: UseStateHandle<String>) -> Html {
    let toggle_asidebar: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        if x.chars().count() > 1 {
            x.set("".to_string());
            article_position.set("".to_string());
        } else {
            x.set("width:250px".to_string());
            article_position.set("margin-left:270px".to_string());
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

            return html!{
                <TitleBar title="current_path/current_file">
                <div
                style="margin-left:60px"
                >
                // <TitleBarButton onclick = {close_window} button_type="close">{""}
                // </TitleBarButton>
                // <TitleBarButton onclick = {toggle_minimize} button_type="minimize">{""}
                // </TitleBarButton>
                // <TitleBarButton onclick={toggle_maximize}button_type="zoom">{""}</TitleBarButton>
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">{""}</TitleBarButton>
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
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">{""}</TitleBarButton>
                </div>
                </TitleBar>
            }
        }
    }
}

