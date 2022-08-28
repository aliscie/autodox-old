use crate::utils::alert;
// use backend::get_users::get_data;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;


use crate::utils::{FileNode, FileMap};
use yewdux::prelude::*;

#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use crate::components::{TitleBar, TitleBarButton};
use crate::router::*;

use crate::components::TreeList;
use web_sys::{window, Document, Element, MouseEvent};
use editor::Editor;


#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());
    let x = aside_bar_taggol.clone();


    let some_data = r#"
     [
         {
             "name":"untitled",
             "id":234,
             "children":[
                 {
                 "name":"my file",
                 "id":235
                 }
             ]
         },
         {
             "name":"my tasks",
             "id":224
         },
         {
             "name":"my notes",
             "id":225
         }
     ]
        "#;
    let (root, dispatch) = use_store::<FileNode>();
    let d = Dispatch::<FileMap>::new();
    dispatch.reduce_mut(move |r| r.children = serde_json::from_str(some_data).unwrap());

    html! {
        <BrowserRouter>
        { crate::utils::get_titlebar(article_position.clone(), x) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL">
            <TreeList files={root.children.clone()} />
        </ul>

        </aside>

        <article style={format!("{}",(*article_position).clone())}>
        <h2 contenteditable="true" class={"heading"}>
          <Switch<Route> render={Switch::render(switch)} />
          </h2>
          <Editor/>
        </article>
        </BrowserRouter>
    }
}

