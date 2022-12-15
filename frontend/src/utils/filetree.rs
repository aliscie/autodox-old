use crate::router::Route;
use crate::specific_components::FileComponent;
use indexmap::IndexSet;
use shared::id::Id;
use shared::log;
use shared::schema::{FileDirectory, FileNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Html};
use yew_hooks::{use_bool_toggle, use_toggle};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_directory: Rc<FileDirectory>,
    pub start: Id,
}

#[function_component(FileTree)]
pub fn to_html(props: &Props) -> Html {
    let map: Rc<RefCell<HashMap<Id, VNode>>> = Rc::new(RefCell::new(HashMap::new()));
    for (id, file_node) in props.file_directory.files.into_iter(props.start) {
        let mut class_name = "";
        let mut has_children = false;
        if let Some(node_ids) = props.file_directory.files.adjacency.get(id) {
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
                file_directory = {props.file_directory.clone()}
                class_name = {class_name}/>
        };
        map.borrow_mut().insert(*id, html_node);
    }
    log!(map.borrow());
    props
        .file_directory
        .files
        .adjacency
        .get(&props.start)
        .unwrap_or(&Vec::new())
        .into_iter()
        .map(|f| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
    //let x = map.borrow().get(&start).unwrap().clone();
    //x
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
    let onclick_file = Callback::from(move |e: MouseEvent| {
        let element: Element = e.target_unchecked_into();
        history.push(&Route::File {
            id: element.id().parse().unwrap(),
        });
    });
    let handle_click_toggle = {
        let display = display.clone();
        Callback::from(move |e: MouseEvent| {
            display.toggle();
        })
    };
    html! {
        <>
            <FileComponent
            key = {props.file_node.id.to_string()}
            id = {props.file_node.id}
            class={format!(" {}",props.class_name)}
            onclickfile = {onclick_file}
            onclick={handle_click_toggle}
            name={props.file_node.name.clone()}/>
            if props.has_children && *display == "active"{
                <ul  class ={"nested active"}>
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
