use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use yew::{html, Html};

use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use yewdux::prelude::*;

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
        let nodes = self.adjacency.get(&start);
        let history = use_history().unwrap();
        if let Some(nodes) = nodes {
            if nodes.len() > 0 {
                let handle_click: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
                    let curr: Element = e.target_unchecked_into();
                    history.push(Route::File { id: start });
                    curr.class_list().toggle("caret-down");
                    let _ = &curr
                        .parent_element()
                        .unwrap()
                        .query_selector(".nested")
                        .unwrap()
                        .unwrap()
                        .class_list()
                        .toggle("active");
                });
                return html! {
                    <>
                    <li onclick={handle_click} class = "caret">{&self.vertices.get(&start).unwrap().name}</li>
                    <ul class = "nested">
                        { nodes.into_iter().map(|id| self.to_html(*id)).collect::<Html>() }
                    </ul>
                    </>
                };
            } else {
                // we got no child simple element
                let handle_click: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
                    history.push(Route::File { id: start });
                });
                return html! {
                    <li  onclick = { handle_click }>{&self.vertices.get(&start).unwrap().name}</li>
                };
            }
        }
        // we got no entry simple element!
        let handle_click: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
            history.push(Route::File { id: start });
        });
        return html! {
            <li  onclick = { handle_click }>{&self.vertices.get(&start).unwrap().name}</li>
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    pub data: String,
}
