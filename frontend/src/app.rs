use crate::router::{switch, Route};
use crate::specific_components::{ButtonsGroup, SearchFilters};
use crate::utils::filetree::FileTree;
use crate::utils::{DeviceInfo, GetTitleBar};
use shared::schema::{FileDirectory, FileNode};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (rc_device_info, _) = use_store::<DeviceInfo>();
    let dispatch_file = Dispatch::<FileDirectory>::new();

    let on_market_place: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        // TODO
        // history.push(Route::File { id: market_page });
    });

    let on_create_file: Callback<MouseEvent> = dispatch_file.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            let file = FileNode::default();
            let x = crate::backend::create_file(
                state.id,
                state.files.root.unwrap(),
                "untitled".to_string(),
                file.id,
            )
            .await;
            if x.is_ok() {
                state
                    .files
                    .push_children(state.files.root.unwrap(), file.id, file);
            }
        })
    });

    let mut aside_style = "";
    if rc_device_info.is_aside {
        aside_style = "width:250px";
    }

    let mut main_style = "";
    if rc_device_info.is_aside
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
                    <ul id="myUL">
                        <FileTree/>
                        <bottom_buttons>
                            <button onclick={on_create_file}><i class="fa-solid fa-plus"></i>{"Add file"}</button>
                            <span><input placeholder="Add from test"/></span>
                            <button onclick={on_market_place}><i class="fa-solid fa-globe"></i>{"Market place"}</button>
                        </bottom_buttons>
                    </ul>
                </aside>
                <main style={format!("transition: 0.2s; margin-top: 35px; {}", main_style)}>
                    <Switch<Route> render= {switch}/>
                </main>
            </div>
        </BrowserRouter>
    }
}
