use crate::render::render;
use shared::id::Id;
use shared::schema::{EditorElementCreate, EditorElementUpdate, ElementTree};
use shared::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{Element, HtmlInputElement, MutationObserver, MutationObserverInit, MutationRecord, Node, Range, window};
use yew::prelude::*;
use yew::{function_component, html};
use yew::virtual_dom::VNode;
use crate::plugins::{EditorToolbar, EditorInsert, CommandItems, DropDownItem, DropDownItemEvent};

/// this captures all the changes in a editor element
#[derive(Debug)]
pub enum EditorChange {
    Update(EditorElementUpdate),
    Create(EditorElementCreate),
    Delete(Id),
}

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub title: String,
    pub element_tree: Rc<RefCell<ElementTree>>,
    pub onchange: Callback<EditorChange>,
}

// this is used for the work space

#[function_component]
pub fn Editor(props: &EditorProps) -> Html {

    // get mouse position and sort it in yewdux
    // each time the mouse move sort the pagex and pagey again

    // get current hovered element and sort it yewdux
    // get the previous  hovered and sorted it in yewdux

    // get the current focused and sorted it
    // get the previous  focused and sorted it in yewdux

    //
    // let state = use_state(|| "".to_string());
    let editor_ref = NodeRef::default();
    let oninput_event = {
        let element_tree = props.element_tree.clone();
        let onchange = props.onchange.clone();
        Closure::wrap(Box::new(
            move |mutation_event: Vec<MutationRecord>, _observer: MutationObserver| {
                for mutation_type in mutation_event {
                    // log!(&format!("{:?}", mutation_type.type_()).into());
                    // log!(&format!("{:?}", mutation_type.target()).into());
                    if let Some(current_element) = mutation_type.target() {
                        match mutation_type.type_().as_ref() {
                            "characterData" => {
                                log!("xxxx");
                                if let Some(parent_element) = current_element.parent_element() {
                                    if let Ok(id) =
                                        Uuid::parse_str(parent_element.id().as_ref()).map(Id::from)
                                    {
                                        // log!(&format!("{:?}", parent_element.inner_html()).into());
                                        // log!(&format!("{:?}", id).into());
                                        let update = EditorElementUpdate {
                                            id,
                                            text: Some(parent_element.inner_html().clone()),
                                            ..Default::default()
                                        };
                                        onchange.emit(EditorChange::Update(update));
                                    }
                                }
                            }
                            "attributes" => {
                                if let Some(parent_element) = current_element.parent_element() {
                                    unimplemented!();
                                    // crate::shared::log!(format!("Got create: {:?}", parent_element.inner_html()));
                                }
                            }
                            "childList" => {
                                let removed_nodes = mutation_type.removed_nodes();
                                for i in 0..removed_nodes.length() {
                                    removed_nodes
                                        .get(i)
                                        .and_then(|node| node.dyn_into::<Element>().ok())
                                        .and_then(|element| {
                                            Uuid::parse_str(element.id().as_str()).ok()
                                        })
                                        .map(|id| onchange.emit(EditorChange::Delete(id.into())));
                                }
                                if removed_nodes.length() > 0 {
                                    unimplemented!();
                                    // move to next mutation record!
                                    // crate::shared::log!("got element delete!");
                                    // crate::shared::log!(mutation_type.removed_nodes());
                                    continue;
                                }
                                let element = current_element.unchecked_into::<Element>();
                                if element.id() == "text_editor" {
                                    continue;
                                }
                                let new_id = Uuid::new_v4();
                                let mut prev_element_id: Option<Id> = None;
                                if let Some(prev_node) = element.previous_sibling() {
                                    let prev_element = prev_node.unchecked_into::<Element>();
                                    unimplemented!();
                                    // crate::shared::log!(format!("previous element id : {:?}", prev_element.id()));
                                    prev_element_id = Uuid::parse_str(prev_element.id().as_str())
                                        .map(Id::from)
                                        .ok();
                                }
                                let element_create = EditorElementCreate {
                                    id: new_id.into(),
                                    text: element.text_content().unwrap_or_default(),
                                    attrs: HashMap::new(),
                                    tree_id: element_tree.as_ref().borrow().id,
                                    parent_id: element_tree
                                        .as_ref()
                                        .borrow()
                                        .elements
                                        .root
                                        .unwrap(),
                                    prev_element_id,
                                    children: None,
                                };
                                onchange.emit(EditorChange::Create(element_create));
                                element.set_id(&new_id.to_string());
                            }
                            anything_else => unimplemented!(), //crate::shared::log!(anything_else),
                        }
                    }
                }
            },
        ) as Box<dyn FnMut(_, _)>)
    };

    use_effect_with_deps(
        move |editor_ref| {
            let mutation_observer =
                MutationObserver::new(oninput_event.as_ref().unchecked_ref()).unwrap();
            //let doc = window().unwrap_throw().document().unwrap_throw();
            //let editor: Rc<Element> = Rc::new(editor_ref.c!(ast::<Element>().unwrap());
            let _ = mutation_observer.observe_with_options(
                &editor_ref.get().unwrap(),
                MutationObserverInit::new()
                    // child attributes or editor attributes chanding
                    .attributes(true)
                    // a new child get created or deleted
                    .child_list(true)
                    // user typed something
                    .character_data(true)
                    .character_data_old_value(true)
                    .subtree(true),
            );
            //PasteConverter::new(editor.clone());
            //TODO
            // DragAndDrop::new(editor.clone());
            // Mention::new(editor.clone(), reg_ex("@\w+"), mentions_components_list); // use the mention plugin to insert mention inline specific_components
            // Mention::new(editor.clone(), "\//w+", components_list); // use the mention plugin for / insert component blocks
            // Mention::new(editor.clone(), "\:/w+",emojis_components_list); // use the mention plugin for : insert emojis inline
            move || {
                drop(oninput_event);
                mutation_observer.disconnect();
            }
        },
        editor_ref.clone(),
    );
    let slash_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {
        log!(event.value);
    });
    let emoji_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {
        // let value: VNode = event.value; // TODO get inner text
        log!("------------------------------------------------------------");
        let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node("❤️"));
    });

    let mention_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {
        log!(event.value);
    });

    let element_tree = props.element_tree.clone();

    let action: Callback<String> = Callback::from(move |e: String| {
        log!(e.clone());
        // onchange.emit(EditorChange::Update(EditorElementUpdate {
        //     id: element_tree.as_ref().borrow().elements.root.unwrap(),
        //     text_format: Some(format),
        //     ..Default::default()
        // }));
    });

    html! {
        <span
            class={css_file_macro!("main.css")}
       >
            <h2 contenteditable="true" class={"editor_title heading"}>
            {props.title.clone()}
        </h2>
            <span
            class = "text_editor_container"
            id = "text_editor_container"
           >

            <EditorToolbar  action={action}/>
            <EditorInsert items={insertion_closures::components()}  trigger={"/".to_string()} command={slash_clouser}/>
            <EditorInsert items={insertion_closures::mentions()}  trigger={"@".to_string()} command={mention_clouser}/>
            <EditorInsert items={insertion_closures::emojies()}  trigger={":".to_string()}  command={emoji_clouser}/>

            <div  ref =  {editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor">
            { render(&element_tree.as_ref().borrow(), element_tree.as_ref().borrow().elements.root.unwrap()) }
        </div>
            </span>
            </span>
    }
}

use shared::schema::*;
use crate::{insertion_closures, plugins};
