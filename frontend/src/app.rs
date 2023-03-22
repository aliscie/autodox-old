use crate::backend;
use crate::router::{switch, Route};
use crate::specific_components::{FilesCategory, SearchFilters};
use crate::utils::filetree::FileTree;
use crate::utils::{DeviceInfo, GetTitleBar};
use editor::plugins::{ContextMenu, Position};
use shared::schema::UserQuery;
use shared::schema::{FileDirectory, FileNode};

use shared::*;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Element, FocusEvent, HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::suspense::*;
use yew_router::prelude::*;
use yewdux::functional::use_store;
use yewdux::prelude::*;

#[derive(PartialEq, Clone)]
pub struct FrontendState {
    pub render_context_menu: Callback<(MouseEvent, Html)>,
}

#[hook]
fn use_load_data() -> SuspensionResult<UseFutureHandle<Result<(), String>>> {
    let dispatch_device_info = Dispatch::<DeviceInfo>::new();

    use_future_with_deps(
        move |_| async move {
            let auth = backend::is_logged().await.as_bool().unwrap();
            let _ = &dispatch_device_info.reduce_mut(|state| state.is_authed = auth);
            let register = backend::register("ali".to_string()).await;
            // let profile_jv = backend::get_profile().await;
            let profile_jv = backend::call_ic_np("get_profile".to_string()).await;
            let profile_res = serde_wasm_bindgen::from_value::<UserQuery>(profile_jv);
            if let Ok(profile_obj) = profile_res {
                let _ = &dispatch_device_info.reduce_mut(|state| state.profile = profile_obj);
            }
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


    let on_create_file: Callback<KeyboardEvent> = dispatch_file_directory
        .reduce_mut_future_callback_with(move |state, _e: KeyboardEvent| {
            Box::pin(async move {
                let input: HtmlInputElement = _e.target_unchecked_into();
                let value: String = input.inner_text();
                if _e.key() == "Enter" {
                    _e.prevent_default();
                };
                if _e.key() != "Enter" {
                    input.class_list().remove_1("tool").unwrap();
                };

                if value.len() == 0 && _e.key() == "Enter" {
                    input.class_list().add_1("tool").unwrap();
                } else if _e.key() == "Enter" {
                    let mut file = FileNode::default();
                    file.name = value.clone();

                    let _ = input.class_list().add_1("loader");
                    let x = crate::backend::create_file(
                        state.id,
                        state.files.root.unwrap(),
                        value,
                        file.id,
                    )
                    .await;
                    if x.is_ok() {
                        state
                            .files
                            .push_children(state.files.root.unwrap(), file.id, file);
                        let _ = input.class_list().remove_1("loader");
                    }
                };
            })
        });

    let mut aside_style = "";
    if rc_device_info.is_aside {
        aside_style = "width:250px";
    }

    let mut main_style = "10%";
    if rc_device_info.is_aside  && window().unwrap_throw().inner_width().unwrap().as_f64().unwrap() > 750 as f64  {
           main_style = "255px";
    };

    let add_file_blur: Callback<FocusEvent> = Callback::from(move |_e: FocusEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_inner_html("Hit enter to add new file.");
    });
    let add_file_focus: Callback<FocusEvent> = Callback::from(move |_e: FocusEvent| {
        let curr: Element = _e.target_unchecked_into();
        curr.set_inner_html("");
    });
    let context_menu_items = use_state(|| html! {});
    let context_menu_position: UseStateHandle<Option<Position>> = use_state_eq(|| None);
    let render_context_menu = {
        let context_menu_position = context_menu_position.clone();
        let context_menu_items = context_menu_items.clone();
        Callback::from(move |(e, items): (MouseEvent, Html)| {
            e.prevent_default();
            context_menu_position.set(Some(Position {
                x: e.page_x().into(),
                y: e.page_y().into(),
            }));
            context_menu_items.set(items);
        })
    };
    let global_state = FrontendState {
        render_context_menu,
    };

    html! {
        <BrowserRouter>
        <ContextProvider<FrontendState> context = {global_state}>
            <div id = "app">
                <GetTitleBar/>
                <aside style={aside_style}>
                    <ul id="aside-content">
                        <SearchFilters/>
                        <FilesCategory/>
                        <div class="files-tree">
                            <FileTree/>
                        </div>
                            <span onfocus={add_file_focus} onblur={add_file_blur} onkeydown={on_create_file} contenteditable="true" class="btn" data-tip="File must have at least 1 character." style="width:100%" > {"Hit enter to add new file."}</span>
                            // <span><input placeholder="Add from test"/></span>
                            <button style="width:100%" onclick={on_market_place}><i class=" fa-solid fa-globe"></i>{"Market place"}</button>
                    </ul>
                </aside>
               <ContextMenu position = {(*context_menu_position).clone()}>
                   {(*context_menu_items).clone()}
               </ContextMenu>
                <main class="main_container" style={format!("transition: 0.2s; margin-top: 35px; margin-left: {}; margin-right:10%;", main_style)}>
                    <Switch<Route> render= {switch}/>
                </main>
            </div>
        </ContextProvider<FrontendState>>
        </BrowserRouter>
    }
}
