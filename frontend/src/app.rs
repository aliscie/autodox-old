use yew::prelude::*;
#[cfg(not(feature = "web"))]
use crate::utils::{invoke, invoke_async};

use crate::router::*;

use crate::utils::{ FileNode, FileTree};
use crate::components::TreeList;
use editor::Editor;
use yewdux::prelude::*;
use yew_router::prelude::*;


#[function_component(App)]
pub fn app() -> Html {
    let aside_bar_taggol = use_state_eq(|| "".to_string());
    let article_position = use_state_eq(|| "".to_string());
    let x = aside_bar_taggol.clone();
    let dispatch = Dispatch::<FileTree>::new();
    dispatch.reduce_mut(|r| {
        r.push_vertex(
            234,
            FileNode {
                id: 234,
                name: "FileOne".into(),
                data: "File one".into(),
            },
        );
        r.push_vertex(
            235,
            FileNode {
                id: 235,
                name: "FileTwo".into(),
                data: "File tow".into(),
            },
        );
        r.push_vertex(
            225,
            FileNode {
                id: 225,
                name: "FileThree".into(),
                data: "File three".into(),
            },
        );
    });
    dispatch.reduce_mut(|r| {
        r.push_edge(0, 234);
        r.push_edge(234, 235);
        r.push_edge(0, 225);
    });

    html! {
        <BrowserRouter>
        <div>
        { super::utils::get_titlebar(article_position.clone(), x) }
        <aside style={format!("{}",(*aside_bar_taggol).clone())}>

        <ul  id="myUL">
            <TreeList/>
        </ul>
        </aside>
        <article style={format!("{}",(*article_position).clone())}>
        <h2 contenteditable="true" class={"heading"}>
          <Switch<Route> render={Switch::render(switch)} />
          </h2>
          <Editor/>
        </article>

        </div>
        </BrowserRouter>
    }
}

