use indexmap::IndexSet;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use shared::Tree;
use uuid::Uuid;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Html};
use yew_router::prelude::*;

use crate::router::Route;
use yewdux::prelude::*;

use crate::app_components::FileComponent;
use shared::schema::{FileDirectory, FileNode};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Store)]
pub struct FileTree {
    pub id: Uuid,
    pub files: Tree<Uuid, FileNode>,
}

impl Default for FileTree {
    fn default() -> Self {
        let mut d = Self::new();
        let id = Uuid::new_v4();
        d.files.push_vertex(
            id,
            FileNode {
                id,
                name: "root".into(),
                element_tree_id: None,
                data: "".into(),
            },
        );
        d.files.root = Some(id);
        return d;
    }
}

impl FileTree {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            files: Tree::new(),
        }
    }
    pub fn to_html(&self, start: Uuid) -> Html {
        let map: Rc<RefCell<HashMap<Uuid, VNode>>> = use_mut_ref(|| HashMap::new());
        let history = use_history().unwrap();
        let onclickfile = Callback::from(move |e: MouseEvent| {
            let element: Element = e.target_unchecked_into();
            history.push(Route::File {
                id: element.id().parse().unwrap(),
            });
        });
        for (id, file_node) in self.files.into_iter(start.clone()) {
            let mut class_name = "";
            let mut has_children = false;
            if let Some(node_ids) = self.files.adjacency.get(id) {
                if node_ids.len() > 0 {
                    has_children = true;
                    class_name = "caret";
                }
            }
            let display = use_state(|| "".to_string());
            let handle_click_toggle = {
                let display = display.clone();
                Callback::from(move |_e: MouseEvent| {
                    // history.push(Route::File { id: start });
                    if has_children {
                        if display.len() == 0 {
                            display.set("active".to_string());
                        } else {
                            display.set("".to_string());
                        }
                    }
                })
            };
            let html_node = html! {
                <>
                    <FileComponent
                        key = {id.to_string()}
                        id = {*id}
                        class={format!(" {}",class_name)}
                        onclickfile = {onclickfile.clone()}
                        onclick={handle_click_toggle}
                        name={file_node.name.clone()}/>
                    if has_children && *display == "active"{
                        <ul  class ={"nested active"}>
                        {
                            self.files.adjacency.get(id)
                                .unwrap()
                                .into_iter()
                                .map(|f| map.borrow().get(f).unwrap().to_owned())
                                .collect::<Html>()
                        }
                        </ul>
                    }
                </>
            };
            map.borrow_mut().insert(*id, html_node);
        }
        self.files
            .adjacency
            .get(&start)
            .unwrap_or(&IndexSet::new())
            .into_iter()
            .map(|f| map.borrow().get(f).unwrap().to_owned())
            .collect::<Html>()
        //let x = map.borrow().get(&start).unwrap().clone();
        //x
    }
}
