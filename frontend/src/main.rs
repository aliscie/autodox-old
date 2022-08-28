use utils::alert;
// use backend::get_users::get_data;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

mod backend;
mod components;
mod extensions;
mod test;
mod utils;
mod router;

#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use components::{TitleBar, TitleBarButton};
use router::*;

use crate::components::TreeList;
use web_sys::{window, Document, Element, MouseEvent};

// mod backend;
// use backend::{get_data};
fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());

    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());
    //spawn_local(async{
    //log_1(&format!("{:?}" ,get_data().await).into());
    //});

    // The effect will run every time `name` changes.
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let message = (*welcome).clone();
    let x = aside_bar_taggol.clone();

    html! {
        <BrowserRouter>
        { get_titlebar(article_position.clone(), x) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL"><TreeList
        // files={files}
        /></ul>

        </aside>

        <article contenteditable="true" style={format!("{}",(*article_position).clone())}>
        <h2 class={"heading"}>{message}</h2>
            <Switch<Route> render={Switch::render(switch)} />
        </article>
        </BrowserRouter>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    if name.contains(' ') {
        let window = window().unwrap();
        window
            .alert_with_message("Error: Name should not contain sapces!")
            .unwrap();
    } else {
        welcome.set(name);
    }
}

fn get_titlebar(article_position : UseStateHandle<String>, x : UseStateHandle<String>) -> Html {
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
                // style="margin-left:60px"
                >
                <TitleBarButton onclick = {close_window} button_type="close">{""}
                </TitleBarButton>
                <TitleBarButton onclick = {toggle_minimize} button_type="minimize">{""}
                </TitleBarButton>
                <TitleBarButton
                onclick={toggle_maximize}
                button_type="zoom">{""}
                </TitleBarButton>
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">{""}
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
                <TitleBar title="current_path/current_file">
                <div
                >
                <TitleBarButton onclick={toggle_asidebar} button_type="toggle">{""}</TitleBarButton>
                </div>
                </TitleBar>
            }
        }
    }
}
