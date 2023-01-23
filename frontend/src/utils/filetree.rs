use crate::router::Route;
use crate::specific_components::FileComponent;
use gloo::console::log;
use indexmap::IndexSet;
use shared::id::Id;
use shared::schema::{FileDirectory, FileNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Html};
use yew_hooks::{use_bool_toggle, use_toggle};
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;
use crate::components::Loading;

#[function_component(FileTree)]
pub fn to_html() -> Html {
    let (tree, _) = use_store::<FileDirectory>();
    let start = tree.clone().files.root.unwrap().clone();
    let map: Rc<RefCell<HashMap<Id, VNode>>> = Rc::new(RefCell::new(HashMap::new()));

    if format!("{:#?}", &tree.clone().files).len()< 355 {
        return html!{<Loading/>}
    }

    for (id, file_node) in tree.clone().files.into_iter(start) {
        let mut class_name = "";
        let mut has_children = false;

        if let Some(node_ids) = tree.clone().files.adjacency.get(id) {
            if !node_ids.is_empty() {
                has_children = true;
                class_name = "caret";
            }
        }

        let html_node = html! {
            <RenderFileElement
                file_node = {file_node.clone()}
                has_children = {has_children}
                map = {map.clone()}
                file_directory = {tree.clone()}
                class_name = {class_name}
            />
        };
        map.borrow_mut().insert(*id, html_node);
    }

    tree.clone()
        .files
        .adjacency
        .get(&start)
        .unwrap_or(&Vec::new())
        .into_iter()
        .map(|f| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
}

#[derive(Properties, PartialEq)]
struct RenderFileElementProps {
    file_node: FileNode,
    has_children: bool,
    map: Rc<RefCell<HashMap<Id, VNode>>>,
    file_directory: Rc<FileDirectory>,
    class_name: &'static str,
}

#[function_component(RenderFileElement)]
fn render_file_element(props: &RenderFileElementProps) -> Html {
    let display = use_toggle("", "active");
    let history = use_navigator().unwrap();

    let on_file = Callback::from(move |e: MouseEvent| {
        let element: Element = e.target_unchecked_into();
        history.push(&Route::File {
            id: element.id().parse().unwrap(),
        });
    });

    let on_toggle = {
        let display = display.clone();
        Callback::from(move |e: MouseEvent| {
            log!("toggle plus".to_string());
            display.toggle();
        })
    };

    html! {
        <>
            <FileComponent
                key = {props.file_node.id.to_string()}
                id = {props.file_node.id}
                class={format!("{}",props.class_name)}
                onclickfile = {on_file}
                onclick={on_toggle}
                name={props.file_node.name.clone()}
            />
            if props.has_children && *display == "active" {
                <ul class={"nested active"}>
                {
                    props.file_directory.files.adjacency.get(&props.file_node.id)
                        .unwrap()
                        .into_iter()
                        .map(|f| props.map.borrow().get(f).unwrap().to_owned())
                        .collect::<Html>()
                }
                </ul>
            }
        </>
    }
}
