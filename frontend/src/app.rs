use wasm_bindgen_futures::spawn_local;
use crate::app_components::{ButtonsGroup, SearchFiltes};
use crate::backend;
use crate::components::TreeList;
use crate::router::{Route, switch};
use editor::Editor;
use yew_router::prelude::*;
use shared::schema::{FileDirectory, FileNode};
use web_sys::{MouseEvent, console};
use yew::prelude::*;
use yewdux::prelude::*;
use shared::log;
use  wasm_bindgen::JsValue;

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async move {
        let canister_id = "rrkah-fqaaa-aaaaa-aaaaq-cai".to_string();
        log!(&backend::read(canister_id.clone()).await);
        log!(JsValue::js_typeof(&backend::read(canister_id.clone()).await));
    });

    let aside_bar_toggle = use_state_eq(|| "".to_string());
    let toggle_aside = aside_bar_toggle.clone();
    let file_dispatch = Dispatch::<FileDirectory>::new();
    // only do it once
    use_effect_with_deps(
        move |_| {
            let x = backend::initialize();
            console::log_1(&format!("{:?}", x).into());
            || {}
        },
        (),
    );

    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });

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
                console::log_1(&format!("create_file response : {:?}", x).into());
                if x.is_ok() {
                    state
                        .files
                        .push_children(state.files.root.unwrap(), file.id, file);
                }
            })
        });

    html! {
        <BrowserRouter>

        <div id = "app">
        { super::utils::get_titlebar(toggle_aside ) }
        <aside style={(*aside_bar_toggle).clone().to_string()}>

        <SearchFiltes/>

        <ButtonsGroup/>

        <ul  id="myUL">
            <TreeList/>
            <bottom_buttons>
                <button onclick={handle_create_file}><i class="fa-solid fa-plus"></i>{"Add file"}</button>
                <span ><input placeholder="Add from test"/></span>
                <button onclick={onclick_market_place} ><i class="fa-solid fa-globe"></i>{"Market place"}</button>
            </bottom_buttons>

        </ul>
        </aside>
        //<main>
            //<Switch<Route> render= {Switch::render(switch)} />
        //</main>
        <Editor title = "text"/>
        </div>
        </BrowserRouter>
    }
}
