use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

use crate::router::Route;
use yewdux::prelude::*;

use crate::components::FileComponent;
use shared::Tree;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Store, Default)]
pub struct FileTree{
    pub files : Tree<u64, FileNode>
}

impl FileTree{
    pub fn to_html(&self, start: u64) -> Html {
        // TODO : make this iterative
        let mut nodes = self.files.adjacency.get(&start);
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
                curr.class_list().toggle("caret-down");
                let _ = &curr
                    .parent_element()
                    .unwrap()
                    .query_selector(".nested")
                    .unwrap()
                    .unwrap()
                    .class_list()
                    .toggle("active");
            }
        });

        // avoid repetition
        // i found `<li class={class_name}  onclick = { handle_click }>{&self.vertices.get(&start).unwrap().name}</li>` was repeated three times
        // now i have `let Some(nodes) = nodes` repeated two times but I don't know how to avoid that.

        let new_file = html! { "+ Create new file."};
        return html! {
        <>
            <FileComponent 
            class={format!("file_component hovering {}",class_name)} 
            onclick={handle_click} 
            name={self.files.vertices.get(&start).unwrap().name.clone()}/>

            if let Some(nodes) = nodes {
                { if has_children{

                    html!{<ul class = "nested">
                        { nodes.into_iter().map(|id| self.to_html(*id)).collect::<Html>()}
                    </ul>}
                } else{
                        new_file
                    }
                }
            }
            else{{
                new_file
            }}
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
