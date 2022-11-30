use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::render::render;
use shared::id::Id;
use shared::schema::{EditorElementCreate, EditorElementUpdate, ElementTree};
use shared::*;
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::console::log_1;
use web_sys::{MutationObserver, MutationObserverInit, MutationRecord, Element};
use yew::prelude::*;
use yew::{function_component, html};

/// this captures all the changes in a editor element
#[derive(Debug)]
pub enum EditorChange {
    Update(EditorElementUpdate),
    Create(EditorElementCreate),
    Delete(Id),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub element_tree: Rc<RefCell<ElementTree>>,
    pub onchange: Callback<EditorChange>,
}

struct EditorState{
    color : String,
    is_bold : bool,
}

// this is used for the work space

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
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
        Closure::wrap(
            Box::new(move |e: Vec<MutationRecord>, _observer: MutationObserver| {
                for i in e {
                    log_1(&format!("{:?}", i.type_()).into());
                    log_1(&format!("{:?}", i.target()).into());
                    match i.type_().as_ref() {
                        "characterData" => {
                            if let Some(x) = i.target() {
                                if let Some(parent_element) = x.parent_element() {
                                    if let Ok(id) =
                                        Uuid::parse_str(parent_element.id().as_ref()).map(Id::from)
                                    {
                                        log_1(&format!("{:?}", parent_element.inner_html()).into());
                                        log_1(&format!("{:?}", id).into());
                                        let update = EditorElementUpdate {
                                            id,
                                            text: Some(parent_element.inner_html().clone()),
                                            ..Default::default()
                                        };
                                        onchange.emit(EditorChange::Update(update));
                                    }
                                }
                            }
                        }
                        "attributes" => {
                            if let Some(x) = i.target() {
                                if let Some(parent_element) = x.parent_element() {
                                    log!(format!("Got attributes change : {:?}", parent_element.inner_html()));
                                }
                            }
                        }
                        "childList" => {
                            if let Some(x) = i.target() {
                                let element = x.unchecked_into::<Element>();
                                log!(format!("got childList for id : {:?}", element.id()));
                                if element.id() == "text_editor" {
                                    continue;
                                }
                                let new_id = Uuid::new_v4();
                                if let Some(prev_node) = element.previous_sibling() {
                                    let prev_element = prev_node.unchecked_into::<Element>();
                                    log!(format!("previous element id : {:?}", prev_element.id()));
                                }
                                let element_create = EditorElementCreate {
                                    id : new_id.into(),
                                    text : element.text_content().unwrap_or_default(),
                                    attrs: HashMap::new(),
                                    tree_id : element_tree.as_ref().borrow().id,
                                    parent_id : element_tree.as_ref().borrow().elements.root.unwrap(),
                                    children : None,
                                };
                                onchange.emit(EditorChange::Create(element_create));
                                element.set_id(&new_id.to_string());
                            }
                        }
                        anything_else => log!(anything_else),
                    }
                }
            }) as Box<dyn FnMut(_, _)>,
        )
    };

    use_effect_with_deps(
        move |editor_ref| {
            //let data = &my_function();
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
            // leaking memory here!
            oninput_event.forget();
            //PasteConverter::new(editor.clone());
            //TODO
            // DragAndDrop::new(editor.clone());
            // Mention::new(editor.clone(), reg_ex("@\w+"), mentions_components_list); // use the mention plugin to insert mention inline app_components
            // Mention::new(editor.clone(), "\//w+", components_list); // use the mention plugin for / insert component blocks
            // Mention::new(editor.clone(), "\:/w+",emojis_components_list); // use the mention plugin for : insert emojis inline
            move || {
                mutation_observer.disconnect();
            }
        },
        editor_ref.clone(),
        );

    let element_tree = props.element_tree.clone();

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
            <div contenteditable="false" id="selection-popper" class="buttons_group_class">
            <span class="btn"><i class="fa-bold"></i></span>
            <span class="btn"><i class="fa-italic"></i></span>
            <span class="btn"><i class="fa-paint-roller"></i></span>
            <span class="btn"><i class="fa-comment"></i></span>
            <span class="btn"><i class="fa-droplet"></i></span>
            </div>

            <div  ref =  {editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor">
            { render(&element_tree.as_ref().borrow(), element_tree.as_ref().borrow().elements.root.unwrap()) }
        </div>
            </span>
            </span>
    }
}
