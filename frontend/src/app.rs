use editor::Editor;
use gloo::console::console;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Document, Element, MouseEvent, window};
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux::prelude::*;
use web_sys::console::log_1;
use crate::components::{TitleBar};
use crate::components::TreeList;
use crate::router::*;
use crate::utils::{FileNode, FileTree};

#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let x = aside_bar_taggol.clone();
    let dispatch = Dispatch::<FileTree>::new();
    dispatch.reduce_mut(|r| {
        r.files.push_vertex(
            234,
            FileNode {
                id: 234,
                name: "FileOne".into(),
                data: "File one".into(),
            },
        );
        r.files.push_vertex(
            235,
            FileNode {
                id: 235,
                name: "FileTwo".into(),
                data: "File tow".into(),
            },
        );
        r.files.push_vertex(
            225,
            FileNode {
                id: 225,
                name: "FileThree".into(),
                data: "File three".into(),
            },
        );
        r.files.push_vertex(
            226,
            FileNode {
                id: 226,
                name: "File4".into(),
                data: "File 4".into(),
            },
        );
    });
    dispatch.reduce_mut(|r| {
        r.files.push_edge(0, 234);
        r.files.push_edge(234, 235);
        r.files.push_edge(0, 225);
        r.files.push_edge(0, 226);
    });


    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });

    html! {
        <BrowserRouter>

        <div id = "app">
        { super::utils::get_titlebar( x) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <span style="margin:10px; display:flex;">
            <input type="text" placeholder="Search.." name="search"/>
            <i  class="btn fa fa-search"/>
        </span>

        <ul  id="myUL">
            <TreeList/>
            <bottom_buttons>
                <button ><i class="fa-solid fa-plus"></i>{"Add file"}</button>
                <button onclick={onclick_market_place} ><i class="fa-solid fa-globe"></i>{"Market place"}</button>
            </bottom_buttons>

        </ul>
        </aside>
            <h2 contenteditable="true" class={"editor_title heading"}>
            <Switch<Route> render={Switch::render(switch)} />
            </h2>
      <Editor/>


        </div>
        </BrowserRouter>
    }
}

