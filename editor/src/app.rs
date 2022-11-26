extern crate web_sys;

use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, MutationObserver, MutationObserverInit, MutationRecord, window};
use web_sys::console::log_1;
use yew::{function_component, html};
use yew::prelude::*;


use shared::schema::{EditorElementCreate, EditorElementUpdate, ElementTree};
use shared::*;
use shared::schema::ElementTree;

use crate::plugins::PasteConverter;
use crate::render::render;

/// this captures all the changes in a editor element
#[derive(Debug)]
pub enum EditorChange {
    Update(EditorElementUpdate),
    Create(EditorElementCreate),
    Delete(Uuid),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub element_tree: Rc<RefCell<ElementTree>>,
    pub onchange: Callback<EditorChange>,
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
    use_effect_with_deps(move |_my_text| {
        let timeout = Timeout::new(250, move || {
            // state.set(props.element_tree.clone);
            log!("xxx");
        });

        timeout.forget();
        || {}
    }, props.element_tree.clone());


    let empty = "empty".to_string();
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
                                    if let Ok(id) = Uuid::parse_str(parent_element.id().as_ref()) {
                                        log_1(&format!("{:?}", parent_element.inner_html()).into());
                                        log_1(&format!("{:?}", id).into());
                                        if let Some(element) = element_tree
                                            .as_ref()
                                            .borrow_mut()
                                            .elements
                                            .vertices
                                            .get_mut(&id)
                                        {
                                            element.text = parent_element.inner_html();
                                            let update = EditorElementUpdate {
                                                id: element.id,
                                                text: Some(parent_element.inner_html()),
                                                ..Default::default()
                                            };
                                            onchange.emit(EditorChange::Update(update));
                                        }
                                    }
                                }
                            }
                        }
                        anything_else => log_1(&anything_else.into()),
                    }
                }
            }) as Box<dyn FnMut(_, _)>,
        )
    };

    use_effect_with_deps(
        move |_my_text| {
            //let data = &my_function();
            let mutation_observer =
                MutationObserver::new(oninput_event.as_ref().unchecked_ref()).unwrap();
            let doc = window().unwrap_throw().document().unwrap_throw();
            let editor: Rc<Element> = Rc::new(doc.get_element_by_id("text_editor").unwrap_throw());
            let _ = mutation_observer.observe_with_options(
                &editor.get_root_node(),
                MutationObserverInit::new()
                    .attributes(true)
                    .child_list(true)
                    .character_data(true)
                    .character_data_old_value(true)
                    .subtree(true),
            );
            // leaking memory here!
            oninput_event.forget();
            PasteConverter::new(editor.clone());
            //TODO
            // DragAndDrop::new(editor.clone());
            // Mention::new(editor.clone(), reg_ex("@\w+"), mentions_components_list); // use the mention plugin to insert mention inline app_components
            // Mention::new(editor.clone(), "\//w+", components_list); // use the mention plugin for / insert component blocks
            // Mention::new(editor.clone(), "\:/w+",emojis_components_list); // use the mention plugin for : insert emojis inline
            move || {
                mutation_observer.disconnect();
            }
        },
        empty,
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
            >
            <div contenteditable="false" id="selection-popper" class="buttons_group_class">
            <span class="btn"><i class="fa-bold"></i></span>
            <span class="btn"><i class="fa-italic"></i></span>
            <span class="btn"><i class="fa-paint-roller"></i></span>
            <span class="btn"><i class="fa-comment"></i></span>
            <span class="btn"><i class="fa-droplet"></i></span>
            </div>

            <div /* ref =  {editor_ref} */ contenteditable = "true" class="text_editor" id = "text_editor">
            { render(&element_tree.as_ref().borrow(), element_tree.as_ref().borrow().elements.root.unwrap()) }
        </div>
            </span>
            </span>
    }
}
