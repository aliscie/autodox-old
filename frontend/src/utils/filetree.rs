use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use yew::{html, Html};

use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use yewdux::prelude::*;


#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}


#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    html! {
        <li onclick = { props.onclick.clone() } style="flex: 1 1 auto; white-space: nowrap; min-width: 0px; overflow: hidden; text-overflow: ellipsis; display: flex; align-items: center;"><div class="notranslate" style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">{props.children.clone()}</div></li>
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Store)]
pub struct FileTree {
    pub vertices: HashMap<u64, FileNode>,
    pub adjacency: HashMap<u64, HashSet<u64>>,
}

impl Default for FileTree {
    fn default() -> Self {
        let mut d = Self::new();
        d.push_vertex(
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
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
    pub fn push_vertex(&mut self, id: u64, vertex: FileNode) {
        self.vertices.insert(id, vertex);
    }
    pub fn push_edge(&mut self, from: u64, to: u64) {
        let adjacency_to_from = self.adjacency.entry(from).or_default();
        adjacency_to_from.insert(to);
    }

    pub fn remove(&mut self, id: &u64) {
        self.vertices.remove(id);
        self.adjacency.remove(id);
        for (_id, children) in self.adjacency.iter_mut() {
            children.remove(id);
        }
    }
    pub fn to_html(&self, start: u64) -> Html {
        // TODO : make this iterative
        let mut nodes = self.adjacency.get(&start);
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
        return html! {
        <>
            <li class={class_name}  onclick = { handle_click }>{&self.vertices.get(&start).unwrap().name}</li>
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

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    pub data: String,
}
