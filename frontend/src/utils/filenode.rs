use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yew::{Html, html};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Element, MouseEvent, window, Document};
use yew_router::prelude::*;

// TODO
//  Can we make this global (we should not need to use this line everytime we need it in a file)
use crate::{extensions::TargetElement, router::Route};
use yewdux::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Store, Clone)]
pub struct FileNode {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(id: u64, name: String) -> FileNode {
        FileNode {
            id,
            name,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child: FileNode) {
        self.children.push(child);
    }
    pub fn navigate<'a>(&'a self) -> FileNavigableNode {
        FileNavigableNode {
            node: self,
            parent: None,
        }
    }
    fn handle_click(e : MouseEvent){
        let curr = e.target_element().unwrap();
        curr.class_list().toggle("caret-down");
    }
    pub fn to_html(&self) -> Html {
        let id = self.id.clone();
        let history = use_history().unwrap();
        if self.children.len() > 0 {
            let handle_click: Callback<MouseEvent> = Callback::from(move |e: MouseEvent| {
                let curr = e.target_element().unwrap();
                history.push(Route::File{ id });
                curr.class_list().toggle("caret-down");
                let _ = &curr.parent_element().unwrap().query_selector(".nested").unwrap().unwrap().class_list().toggle("active");
            });

            return html! {
                <>
                <li onclick={handle_click} class = "caret">{&self.name}</li>
                <ul class = "nested">{
                    (&self.children).into_iter().map(|file| file.to_html()).collect::<Html>()
                } </ul>
                </>
            };
        } else {
            let handle_click: Callback<MouseEvent> = Callback::from(move |_e : MouseEvent| {
                history.push(Route::File{ id });
            });
            return html! {
                <li  onclick = { handle_click }>{&self.name}</li>
            };
        }
    }
}

/// Type for storing file data we will get it from backend
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Store, Clone)]
pub struct FileMap{
    pub data : HashMap<u64, String>,
}

/// For easier navigation
#[derive(Debug)]
pub struct FileNavigableNode<'a> {
    pub node: &'a FileNode,
    pub parent: Option<&'a FileNavigableNode<'a>>,
}

impl<'a> FileNavigableNode<'a> {
    pub fn child(&self, index: usize) -> FileNavigableNode {
        FileNavigableNode {
            node: &self.node.children[index],
            parent: Some(self),
        }
    }
}

