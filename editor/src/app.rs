extern crate web_sys;

use shared::*;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DragEvent, Element, MouseEvent, window};
use yew::{function_component, html};
use yew::prelude::*;
use yewdux::prelude::{Dispatch};

use crate::element_tree::{Attrs, EditorElement, ElementTree};
use crate::plugins::PasteConverter;
use crate::utils::my_function;

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
        web_sys::console::log_1(&my_function().into());

        let doc = window().unwrap_throw().document().unwrap_throw();
        let editor: Rc<Element> = Rc::new(doc.query_selector(".text_editor").unwrap_throw().unwrap_throw());
        PasteConverter::new(editor.clone());
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
                r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Maxime mollitia,
                molestiae quas vel sint commodi repudiandae consequuntur voluptatum laborum
                numquam blanditiis harum quisquam eius sed odit fugiat iusto fuga praesentium
                optio, eaque rerum! Provident similique accusantium nemo autem. Veritatis
                obcaecati tenetur iure eius earum ut molestias architecto voluptate aliquam
                nihil, eveniet aliquid culpa officia aut! Impedit sit sunt quaerat, odit,
                tenetur error, harum nesciunt ipsum debitis quas aliquid. Reprehenderit,
                quia. Quo neque error repudiandae fuga? Ipsa laudantium molestias eos
                sapiente officiis modi at sunt excepturi expedita sint? Sed quibusdam
                recusandae alias error harum maxime adipisci amet laborum. Perspiciatis
                minima nesciunt dolorem! Officiis iure rerum voluptates a cumque velit
                quibusdam sed amet tempora. Sit laborum ab, eius fugit doloribus tenetur
                fugiat, temporibus enim commodi iusto libero magni deleniti quod quam
                consequuntur! Commodi minima excepturi repudiandae velit hic maxime
                doloremque. Quaerat provident commodi consectetur veniam similique ad
                earum omnis ipsum saepe, voluptas, hic voluptates pariatur est explicabo
                fugiat, dolorum eligendi quam cupiditate excepturi mollitia maiores labore
                suscipit quas? Nulla, placeat. Voluptatem quaerat non architecto ab laudantium
                modi minima sunt esse temporibus sint culpa, recusandae aliquam numquam
                totam ratione voluptas quod exercitationem fuga. Possimus quis earum veniam
                uasi aliquam eligendi, placeat qui corporis!
                "#
                    .to_string(),
                HashMap::new(),
            ),
        );
    });

    // web_sys::console::log_1(&serde_json::to_string(element_tree_dispatch.get().as_ref()).unwrap().into());

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

            // TODO instead of  { 'value': 'bold text', 'attrs': {'bold': true} },
            //  you MUST use { 'text': 'bold', 'attrs': {'style': font-weight: bold;} }
            //  Because this will help reduce the amount of code required for rendering and conversion.
            //  rendering mean: convert from the database to html
            //  conversion: is to convert to html into the database


            { element_tree_dispatch.get().to_html(0) }
        </div>
    </span>
    </span>
    }
}
