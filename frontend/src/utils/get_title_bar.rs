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
    let web_and_desk_title_bar: Html = html! {
    <>
        <div class="buttons">
        <TitleBarButton onclick={toggle_asidebar} button_type="toggle">
        {if is_expanded > 1{"X"} else{"="}}
        </TitleBarButton>
        </div>
    </>
     };

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "web"))]{
        // let toggle_maximize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        //     let _ = invoke::<String>("maximize_window".into(), None).map_err(|e| alert(&e));
        // });
        // let toggle_minimize: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        //     let _ = invoke::<String>("minimize_window".into(), None).map_err(|e| alert(&e));
        // });
        // let close_window: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
        //     let _ = invoke::<String>("close_window".into(), None).map_err(|e| alert(&e));
        // });

            return html!{
                <TitleBar title={current_directory}>
                {web_and_desk_title_bar}
                </TitleBar>
            }
        }
        else {
            return html!{
            <TitleBar title={current_directory}>
            {web_and_desk_title_bar}
            <div style="position:fixed;right:200px;">
                <button>{"Download"}</button>
            </div>

            </TitleBar>
            }
        }
    }
}

