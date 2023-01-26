use crate::backend;
use crate::router::{switch, Route};
use crate::specific_components::{FilesCategory, SearchFilters};
use crate::utils::filetree::FileTree;
use crate::utils::{DeviceInfo, GetTitleBar};
use shared::schema::UserQuery;
use shared::schema::{FileDirectory, FileNode};
use shared::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, MouseEvent, HtmlInputElement};
use yew::prelude::*;
use yew::suspense::*;
use yew_router::prelude::*;
use yewdux::functional::use_store;
use yewdux::prelude::*;

#[hook]
fn use_load_data() -> SuspensionResult<UseFutureHandle<Result<(), String>>> {
    let dispatch_device_info = Dispatch::<DeviceInfo>::new();

    use_future_with_deps(
        move |_| async move {
            let auth = backend::is_logged().await.as_bool().unwrap();
            log!(auth);
            let _ = &dispatch_device_info.reduce_mut(|state| state.is_authed = auth);
            let register = backend::register("ali".to_string()).await;
            log!(register);
            let profile_res = backend::get_profile().await;
            // log!(&profile_res);
            let profile_obj: UserQuery = serde_wasm_bindgen::from_value(profile_res)
                .map_err(|e| String::from("serde error"))?;
            // log!(&profile_obj);
            let _ = &dispatch_device_info.reduce_mut(|state| state.profile = profile_obj);
            let _ = crate::hooks::init_files().await;
            return Ok(());
        },
        (),
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let _ = use_load_data();

    let (rc_device_info, _) = use_store::<DeviceInfo>();
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();

    let on_market_place: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        // TODO
        // history.push(Route::File { id: market_page });
    });

    let on_create_file: Callback<KeyboardEvent> = dispatch_file_directory.reduce_mut_future_callback_with(move |state, _e: KeyboardEvent| {
        let input: HtmlInputElement = _e.target_unchecked_into();
        let value: String = input.value();

        Box::pin(async move {
            if _e.key() == "Enter" {
                let mut file = FileNode::default();
                file.name = value.clone();
                let _ = input.class_list().toggle("loader");
                let x = crate::backend::create_file(
                    state.id,
                    state.files.root.unwrap(),
                    value,
                    file.id,
                )
                    .await;
                if x.is_ok() {
                    state.files.push_children(state.files.root.unwrap(), file.id, file);
                    let _ = input.class_list().toggle("loader");
                }
            };
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
                    <FilesCategory/>
                    <ul id="myUL">
                        <FileTree/>
                        <bottom_buttons>
                            <input value="Add new file." onkeydown={on_create_file}/>
                            // <span><input placeholder="Add from test"/></span>
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
