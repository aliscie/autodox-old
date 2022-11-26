use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use indexmap::IndexSet;
use uuid::Uuid;
use web_sys::{Element, MouseEvent};
use yew::{html, Html};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::use_navigator;
use shared::schema::FileDirectory;
use crate::app_components::FileComponent;
use crate::router::Route;
use shared::id::Id;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub file_directory: Rc<FileDirectory>,
    pub start: Id
}

#[function_component(FileTree)]
pub fn to_html(props : &Props) -> Html {
    let map: Rc<RefCell<HashMap<Uuid, VNode>>> = Rc::new(RefCell::new(HashMap::new()));
    let history = use_navigator().unwrap();
    let onclick_file = Callback::from(move |e: MouseEvent| {
        let element: Element = e.target_unchecked_into();
        history.push(&Route::File {
            id: element.id().parse().unwrap(),
        });
    });
    for (id, file_node) in props.file_directory.files.into_iter(props.start) {
        let mut class_name = "";
        let mut has_children = false;
        if let Some(node_ids) = props.file_directory.files.adjacency.get(id) {
            if !node_ids.is_empty() {
                has_children = true;
                class_name = "caret";
            }
        }
        let display = Rc::new(RefCell::new(""));
        let handle_click_toggle = {
            let display = display.clone();
            Callback::from(move |_e: MouseEvent| {
                // history.push(Route::File { id: start });
                if has_children {
                    if display.borrow().len() == 0 {
                        *display.borrow_mut() = "active";
                        //display.set("active".to_string());
                    } else {
                        //display.set("".to_string());
                        *display.borrow_mut() = "";
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
                    onclickfile = {onclick_file.clone()}
                    onclick={handle_click_toggle}
                    name={file_node.name.clone()}/>
                if has_children && *display.borrow() == "active"{
                    <ul  class ={"nested active"}>
                    {
                        props.file_directory.files.adjacency.get(id)
                            .unwrap()
                            .into_iter()
                            .map(|f| map.borrow().get(f).unwrap().to_owned())
                            .collect::<Html>()
                    }
                    </ul>
                }
            </>
        };
        map.borrow_mut().insert(**id, html_node);
    }
    props.file_directory
        .files
        .adjacency
        .get(&props.start)
        .unwrap_or(&IndexSet::new())
        .into_iter()
        .map(|f| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
    //let x = map.borrow().get(&start).unwrap().clone();
    //x
}
