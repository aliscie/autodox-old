use crate::utils::alert;
// use backend::get_users::get_data;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use crate::components::{TitleBar, TitleBarButton};
use crate::router::*;

use crate::components::TreeList;
use web_sys::{window, Document, Element, MouseEvent};




#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());
    let x = aside_bar_taggol.clone();

    html! {
        <BrowserRouter>
        { crate::utils::get_titlebar(article_position.clone(), x) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL"><TreeList
        // files={files}
        /></ul>

        </aside>

        <article contenteditable="true" style={format!("{}",(*article_position).clone())}>
        <h2 class={"heading"}>
          <Switch<Route> render={Switch::render(switch)} />
          </h2>
         <span>{"text editor is here"}</span>
        </article>
        </BrowserRouter>
    }
}

