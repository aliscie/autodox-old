use crate::backend;
use crate::router::{switch, Route};
use crate::specific_components::{ButtonsGroup, SearchFilters};
use crate::utils::filetree::FileTree;
use crate::utils::{DeviceInfo, GetTitleBar};
use shared::log;
use shared::schema::{FileDirectory, FileNode};
use wasm_bindgen::JsValue;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // use_effect_with_deps(
    //     move |_| {
    //         spawn_local(async move {
    //             let _ = crate::hooks::init_files().await;
    //         });
    //         || {}
    //     },
    //     (),
    // );

    let (device, dispatch) = use_store::<DeviceInfo>();
    // &dispatch.reduce_mut(|state| state.is_aside = true);
    // &dispatch.reduce_mut(|state| state.is_aside = false);
    spawn_local(async move {
        // log!(&backend::read().await);
        // log!(JsValue::js_typeof(
        //     &backend::read().await
        // ));
    });
    // let aside_bar_toggle = use_state_eq(|| "".to_string());
    // let toggle_aside = aside_bar_toggle.clone();
    let file_dispatch = Dispatch::<FileDirectory>::new();

    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });
    // log!( &device.is_aside);
    let handle_create_file: Callback<MouseEvent> =
        file_dispatch.reduce_mut_future_callback(|state| {
            Box::pin(async move {
                let file = FileNode::default();
                let x = crate::backend::create_file(
                    state.id,
                    state.files.root.unwrap(),
                    "untitled".to_string(),
                    file.id,
                )
                    .await;
                // console::log_1(&format!("create_file response : {:?}", x).into());
                if x.is_ok() {
                    state
                        .files
                        .push_children(state.files.root.unwrap(), file.id, file);
                }
            })
        });
    let mut aside_style = "";
    if device.is_aside {
        aside_style = "width:250px";
    }

    let mut main_style = "";
    if device.is_aside
        && window()
        .unwrap_throw()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap()
        > 750 as f64
    {
        main_style = "margin-left:250px";
    }
    html! {
        <BrowserRouter>

            <div id = "app">
            <GetTitleBar/>
            <aside style={aside_style}>

            <SearchFilters/>

            <ButtonsGroup/>

            <ul  id="myUL">
            <FileTree/>
            <bottom_buttons>
            <button onclick={handle_create_file}><i class="fa-solid fa-plus"></i>{"Add file"}</button>
            <span ><input placeholder="Add from test"/></span>
            <button onclick={onclick_market_place} ><i class="fa-solid fa-globe"></i>{"Market place"}</button>
            </bottom_buttons>

            </ul>
            </aside>
            <main style={format!("transition: 0.2s;; margin-top: 35px; {}",main_style)}>
                <Switch<Route> render= {switch} />
            </main>
            //<Editor title = "text" element_tree = { element_tree }/>
            </div>
        </BrowserRouter>
    }
}
