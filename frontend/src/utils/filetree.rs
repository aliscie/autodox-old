use shared::Tree;
use serde::{Deserialize, Serialize};
use yew::{html, Html};
use web_sys::{window, Element, MouseEvent, DragEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use yewdux::prelude::*;

use crate::components::FileComponent;



#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Store)]
pub struct FileTree {
    pub files : Tree<u64, FileNode>,
}

impl Default for FileTree {
    fn default() -> Self {
        let mut d = Self::new();
        d.files.push_vertex(
            0,
            FileNode {
                id: 0,
                name: "root".into(),
                data: "".into(),
            },
        );
        return d;
    }
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            files : Tree::new(),
        }
    }
    pub fn to_html(&self, start: u64) -> Html {
        // TODO : make this iterative
        let nodes = self.files.adjacency.get(&start);
        let history = use_history().unwrap();


        let mut has_children = false;
        let mut class_name = "";
        if let Some(nodes) = nodes {
            if nodes.len() > 0 {
                has_children = true;
                class_name = "caret"
            }
        }

        let handle_click: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
            history.push(Route::File { id: start });
            if has_children {
                let curr: Element = e.target_unchecked_into();
                let _ = curr.class_list().toggle("caret-down");
                let _ = &curr
                    .parent_element()
                    .unwrap()
                    .query_selector(".nested")
                    .unwrap() //TODO if curr.parent_element().unwrap().query_selector(".nested").unwrap() != None {.unwrap().class_list().toggle("active");}
                    .unwrap()
                    .class_list()
                    .toggle("active");
            }
        });


        // avoid repetition
        // i found `<li class={class_name}  onclick = { handle_click }>{&self.vertices.get(&start).unwrap().name}</li>` was repeated three times
        // now i have `let Some(nodes) = nodes` repeated two times but I don't know how to avoid that.


        return html! {
        <>
            <FileComponent class={format!("file_component hovering {}",class_name)} onclick={handle_click} name={self.files.vertices.get(&start).unwrap().name.clone()}/>

            if let Some(nodes) = nodes {
                { if has_children{

                html!{<ul class = "nested">
                    { nodes.into_iter().map(|id| self.to_html(*id)).collect::<Html>()}
                </ul>}
            } else{ html!{"+ Create new file."}}}
            }
        </>
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    pub data: String,
}
