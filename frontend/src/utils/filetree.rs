use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use indexmap::IndexSet;
use uuid::Uuid;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Html};
use yew_router::prelude::*;

use shared::schema::FileDirectory;

use crate::app_components::FileComponent;
use crate::router::Route;

pub fn to_html(file_directory: &FileDirectory, start: Uuid) -> Html {
    let map: Rc<RefCell<HashMap<Uuid, VNode>>> = use_mut_ref(HashMap::new);
    let history = use_history().unwrap();
    let onclick_file = Callback::from(move |e: MouseEvent| {
        let element: Element = e.target_unchecked_into();
        history.push(Route::File {
            id: element.id().parse().unwrap(),
        });
    });
    for (id, file_node) in file_directory.files.into_iter(start) {
        let mut class_name = "";
        let mut has_children = false;
        if let Some(node_ids) = file_directory.files.adjacency.get(id) {
            if !node_ids.is_empty() {
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
        // TODO
        //  add actor from ./ic_agent.js file
        //  let actor  = createActor().await
        //  let files = actor.read_files().await
        //spawn_local(async move {
        //let canister_id = "rrkah-fqaaa-aaaaa-aaaaq-cai".to_string();
        //let files = read(canister_id).await;
        //log!(files);
        // let file_component = FileComponent::new(actor, file_node);
        // let vnode = yew::utils::document().create_element("div").unwrap();
        // file_component.mount(vnode);
        // map.borrow_mut().insert(id, vnode);
        //});

        let html_node = html! {
            <>
                <FileComponent
                    key = {id.to_string()}
                    id = {*id}
                    class={format!(" {}",class_name)}
                    onclickfile = {onclick_file.clone()}
                    onclick={handle_click_toggle}
                    name={file_node.name.clone()}/>
                if has_children && *display == "active"{
                    <ul  class ={"nested active"}>
                    {
                        file_directory.files.adjacency.get(id)
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
    file_directory
        .files
        .adjacency
        .get(&start)
        .unwrap_or(&IndexSet::new())
        .into_iter()
        .map(|f| map.borrow().get(f).unwrap().to_owned())
        .collect::<Html>()
    //let x = map.borrow().get(&start).unwrap().clone();
    //x
}
