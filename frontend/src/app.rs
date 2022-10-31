use crate::components::TreeList;
use crate::utils::FileTree;
use editor::Editor;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::backend;
use crate::app_components::{ButtonsGroup, SearchFiltes};

#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let toggle_aside = aside_bar_taggol.clone();
    let file_dispatch = Dispatch::<FileTree>::new();
    // only do it once
    use_effect_with_deps(
        move |_| {
            let _x = backend::initialize();
            || {}
        },
        (),
    );

    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });

    let handle_create_file: Callback<MouseEvent> =
        file_dispatch.reduce_mut_future_callback(|state| {
            Box::pin(async move {
                let file = crate::backend::create_file(
                    state.id,
                    state.files.root.unwrap(),
                    "untitled".to_string(),
                )
                    .await;
                if let Ok(f) = file {
                    state
                        .files
                        .push_children(state.files.root.unwrap(), f.id, f);
                }
            })
        });

    return html! {
        <BrowserRouter>

        <div id = "app">
        { super::utils::get_titlebar(toggle_aside ) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

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
        //TODO
        // title="string" instead of <Switch<Route> render={Switch::render(switch)} />
        <Editor title="text"/>
        </div>
        </BrowserRouter>
    };
}
