use editor::Editor;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::components::TreeList;
use crate::router::*;
use crate::utils::{FileNode, FileTree};
use uuid::Uuid;

use crate::app_components::{SearchFiltes, ButtonsGroup};
use crate::*;
use src_ic::say_hello;
use web_sys::console::log_1;

#[function_component(App)]
pub fn app() -> Html {
    let x = say_hello();
    log_1(&format!("say_hello {:?}", x).into());
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let toggle_aside = aside_bar_taggol.clone();
    let dispatch = Dispatch::<FileTree>::new();
    //dispatch.reduce_mut(|r| {
        //let id_1 = Uuid::new_v4();
        //let id_2 = Uuid::new_v4();
        //let id_3 = Uuid::new_v4();
        //let id_4= Uuid::new_v4();
        //r.files.push_vertex(
            //id_1,
            //FileNode {
                //id: id_1,
                //name: "FileOne".into(),
                //data: "File one".into(),
                //element_tree_id : None,
            //},
        //);
        //r.files.push_vertex(
            //id_2,
            //FileNode {
                //id: id_2,
                //name: "FileTwo".into(),
                //element_tree_id : None,
                //data: "File tow".into(),
            //},
        //);
        //r.files.push_vertex(
            //id_3,
            //FileNode {
                //id: id_3,
                //name: "FileThree".into(),
                //data: "File three".into(),
                //element_tree_id : None,
            //},
        //);
        //r.files.push_vertex(
            //id_4,
            //FileNode {
                //id: id_4,
                //name: "File4".into(),
                //data: "File 4".into(),
                //element_tree_id : None,
            //},
        //);
        //let root = r.files.root.unwrap();
        //r.files.push_edge(root, id_1);
        //r.files.push_edge(id_1, id_2);
        //r.files.push_edge(root, id_3);
        //r.files.push_edge(root, id_4);
    //});
    spawn_local(async {
        let _ = crate::backend::files::on_startup().await;
    });


    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });

    let handle_create_file: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
        if *IS_WEB {
            // TODO Kazoya
            //  canister_agent.create_file({ name: "new name", content:"emty content" })
        } else {
            // TODO Aman
            //  database.create_file({ name: "new name", content:"emty content" });
        }

    });

    html! {
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
                <span ><input placeholder="Add from cloud"/></span>
                <button onclick={onclick_market_place} ><i class="fa-solid fa-globe"></i>{"Market place"}</button>
                <button ><i class="fa-solid fa-trash"></i>{"Trash"}</button>
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
