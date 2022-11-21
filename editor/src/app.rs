extern crate web_sys;

use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::console::log_1;
use web_sys::{
    window, DragEvent, Element, MouseEvent,
    MutationObserver, MutationObserverInit, MutationRecord,
};
use yew::prelude::*;
use yew::{function_component, html};
use yewdux::prelude::Dispatch;

use shared::schema::{Attrs, EditorElement, ElementTree};
use shared::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::plugins::PasteConverter;
use crate::render::render;
use shared::log;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

// this is used for the work space

#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    // TODO
    // get mouse position and sort it in yewdux
    // each time the mouse move sort the pagex and pagey again

    // get current hovered element and sort it yewdux
    // get the previous  hovered and sorted it in yewdux

    // get the current focused and sorted it
    // get the previous  focused and sorted it in yewdux
    let empty = "empty".to_string();
    // TODO : this is only a prototype code should be cleaned here!
    let oninput_event = Closure::wrap(Box::new(
        move |e: Vec<MutationRecord>, _observer: MutationObserver| {
            let element_tree = Dispatch::<ElementTree>::new();
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
                                    element_tree.reduce_mut(|f| {
                                        if let Some(element) = f.elements.vertices.get_mut(&id) {
                                            element.text = parent_element.inner_html();
                                        }
                                    });
                                }
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            log_1(&format!("{:?}", element_tree.get()).into());
        },
    ) as Box<dyn FnMut(_, _)>);

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

    let onmousemove = {
        move |e: MouseEvent| {
            // log_1(&format!("xxxxxxxxxxxxxxxxxx {:?}", e.page_y()).into());
            // display.set("display: block".to_string());
        }
    };
    let onchange = {
        move |e: Event| {
            log!("change from main");
        }
    };

    let onmousedown = {
        move |e: MouseEvent| {
            // display.set("display: block".to_string());
        }
    };

    let ondragstart = {
        move |e: DragEvent| {
            // opacity:0.5
        }
    };

    let ondragend = {
        move |e: DragEvent| {
            // opacity:1
        }
    };

    let ondragenter = {
        move |e: DragEvent| {
            // background:lightblue
        }
    };

    let ondragleave = {
        move |e: DragEvent| {
            // background:none
        }
    };
    let element_tree_dispatch = Dispatch::<ElementTree>::new();
    element_tree_dispatch.reduce_mut(|r| {
        let root = r.elements.root.unwrap();
        let id = Uuid::new_v4();
        r.elements.push_children(
            root,
            id.clone(),
            EditorElement::new(
                id,
                "bold text".to_string(),
                HashMap::from([(Attrs::Style, "font-weight: bold;".to_string())]),
            ),
        );
        let id = Uuid::new_v4();
        r.elements.push_children(
            root,
            id,
            EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
        );
    });

    let oninput = Callback::from(|e: InputEvent| {
        log_1(&format!("{:?}", e).into());
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
            >


            <div contenteditable="false" id="selection-popper" class="buttons_group_class">
            <span class="btn"><i class="fa-solid fa-bold"></i></span>
            <span class="btn"><i class="fa-solid fa-italic"></i></span>
            <span class="btn"><i class="fa-solid fa-paint-roller"></i></span>
            <span class="btn"><i class="fa-solid fa-comment"></i></span>
            <span class="btn"><i class="fa-solid fa-droplet"></i></span>
            </div>

            <div /* ref =  {editor_ref} */ contenteditable = "true" class="text_editor" id = "text_editor">
            { render(&element_tree_dispatch.get(), element_tree_dispatch.get().elements.root.unwrap()) }
        </div>
            </span>
            </span>
    }
}
