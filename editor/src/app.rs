extern crate web_sys;

use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::{function_component, html};
use yew::prelude::*;
use yewdux::prelude::Dispatch;

use crate::element_tree::{Attrs, EditorElement, ElementTree};
use crate::plugins::{PasteConverter};
use crate::utils::my_function;
use shared::*;
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
    use_effect_with_deps(move |_my_text| {
        let data = &my_function();

        let doc = window().unwrap_throw().document().unwrap_throw();
        let editor: Rc<Element> = Rc::new(doc.query_selector(".text_editor").unwrap_throw().unwrap_throw());
        PasteConverter::new(editor.clone());
        //TODO
        // DragAndDrop::new(editor.clone());
        // Mention::new(editor.clone(), reg_ex("@\w+"), mentions_components_list); // use the mention plugin to insert mention inline app_components
        // Mention::new(editor.clone(), "\//w+", components_list); // use the mention plugin for / insert component blocks
        // Mention::new(editor.clone(), "\:/w+",emojis_components_list); // use the mention plugin for : insert emojis inline
        || {}
    }, empty);


    let onmousemove = {
        move |e: MouseEvent| {
            // log_1(&format!("xxxxxxxxxxxxxxxxxx {:?}", e.page_y()).into());
            // display.set("display: block".to_string());
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
        r.elements
            .push_vertex(0, EditorElement::new(0, "".to_string(), HashMap::new()));
        r.elements.push_children(
            0,
            1,
            EditorElement::new(
                1,
                "bold text".to_string(),
                HashMap::from([(Attrs::Style, "font-weight: bold;".to_string())]),
            ),
        );
        r.elements.push_children(
            0,
            2,
            EditorElement::new(
                2,
                r#"Element is here."#
                    .to_string(),
                HashMap::new(),
            ),
        );
    });


    html! {
    <span
        class={css_file_macro!("main.css")}
     >

     <h2 contenteditable="true" class={"editor_title heading"}>
        {props.title.clone()}
    </h2>

    <span
    {onmousemove}
    contenteditable="true"
    class="text_editor_container"
    >


    <div contenteditable="false" id="selection-popper" class="buttons_group_class">
            <span class="btn"><i class="fa-solid fa-bold"></i></span>
            <span class="btn"><i class="fa-solid fa-italic"></i></span>
            <span class="btn"><i class="fa-solid fa-paint-roller"></i></span>
            <span class="btn"><i class="fa-solid fa-comment"></i></span>
            <span class="btn"><i class="fa-solid fa-droplet"></i></span>
    </div>

        <div  class="text_editor" >
            { element_tree_dispatch.get().to_html(0) }
        </div>
    </span>
    </span>
    }
}
