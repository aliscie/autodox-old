use crate::handle_mutation::handle_mutation;
use crate::insertion_closures;
use crate::plugins::{CommandItems, DropDownItem, EditorInsert, EditorToolbar};
use crate::render::render;
use crate::utils::on_slash_input;
use serde::{Deserialize, Serialize};
use shared::id::Id;
use shared::schema::{
    EditorChange, EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree,
};
use shared::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    window, Element, MutationObserver, MutationObserverInit, MutationRecord, Node, Range,
};
use yew::prelude::*;
use yew::{function_component, html};

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub title: String,
    pub element_tree: Rc<RefCell<ElementTree>>,
    pub onchange: Callback<EditorChange>,
}

// this is used for the work space


use emojic::text::EmojiTextParser;

fn search_for_emojis(search_term: &str) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    // get list of all emoji aliases
    for i in EmojiTextParser::new(":sandwich:").take(10 as usize) {
        log!(i);
    };
    matches.push(EmojiTextParser::new(":cat:").to_string());
    matches
}


#[function_component]
pub fn Editor(props: &EditorProps) -> Html {
    log!(
        search_for_emojis("smile")
    );
    // get mouse position and sort it in yewdux
    // each time the mouse move sort the pagex and pagey again

    // get current hovered element and sort it yewdux
    // get the previous  hovered and sorted it in yewdux

    // get the current focused and sorted it
    // get the previous  focused and sorted it in yewdux

    //
    // let state = use_state(|| "".to_string());
    let toggle = use_force_update();
    let editor_ref = NodeRef::default();
    let oninput_event = {
        let element_tree = props.element_tree.clone();
        let onchange = props.onchange.clone();
        Closure::wrap(Box::new(
            move |mutation_event: Vec<MutationRecord>, _observer: MutationObserver| {
                handle_mutation(&mutation_event, &onchange, element_tree.clone());
            },
        ) as Box<dyn FnMut(_, _)>)
    };

    use_effect_with_deps(
        move |editor_ref| {
            let mutation_observer =
                MutationObserver::new(oninput_event.as_ref().unchecked_ref()).unwrap();
            //let doc = window().unwrap_throw().document().unwrap_throw();
            //let editor: Rc<Element> = Rc::new(editor_ref.c!(ast::<Element>().unwrap());

            // TODO
            //  nested update is a problem
            //  If we create a nested element
            //   It is coming as update on the parent element.
            //   It should show as a create event on the root

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

    let element_tree = props.element_tree.clone();

    let onkeydown: Callback<KeyboardEvent> = Callback::from(move |_e: KeyboardEvent| {
        if _e.key() == "Tab" {
            _e.prevent_default();
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
            let _ = html_document
                .exec_command_with_show_ui_and_value("InsertText", false, "    ")
                .unwrap();
        }
    });
    // TODO make the commands Callback<DropDownItem, Option<Range>> instead of fn(DropDownItem, Option<Range>)
    let emojis_command: fn(DropDownItem, Option<Range>) = (|event, range| {
        // let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node(&event.value));
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let _ = html_document
            .exec_command_with_show_ui_and_value("InsertText", false, &event.value)
            .unwrap();
    });
    let slash_command = {
        let element_tree = element_tree.clone();
        Callback::from(move |(event, range)| {
            on_slash_input(event, range, element_tree.clone());
            toggle.force_update();
        })
    };
    let action: Callback<String> = Callback::from(move |e: String| {
        // log!(e.clone());
        // onchange.emit(EditorChange::Update(EditorElementUpdate {
        //     id: element_tree.as_ref().borrow().elements.root.unwrap(),
        //     text_format: Some(format),
        //     ..Default::default()
        // }));
    });

    let mention_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {});

    // let format_command: fn(String, selectoin) -> Option<()> =  (|event, range| return Some((
    //     onchange.emit(EditorChange::Update(update)); // TODO this should be the same for  on_slash_input, mention_clouser and emojis_command
    //     )));

    html! {
        <span
            class={css_file_macro!("main.css")}
       >
            <h2 contenteditable="true" class={"editor_title heading"}>
            {props.title.clone()}
        </h2>
            <span
            {onkeydown}
            class = "text_editor_container"
            id = "text_editor_container"
           >
            <EditorToolbar
                editor_ref = { editor_ref.clone()}
            // command={Callback::from(move |(e, r)| format_command(e, r))}
            />
            <EditorInsert
                items={insertion_closures::components()}
                trigger={"/".to_string()}
                command={slash_command}/>
            <EditorInsert
                items={insertion_closures::mentions()}
                trigger={"@".to_string()}
                command={Callback::from(move |(e, r)| mention_clouser(e, r))}/>
            <EditorInsert
                items={insertion_closures::emojies()}
                trigger={":".to_string()}
                command={Callback::from(move |(e, r) | emojis_command(e, r))}/>
            <div  ref =  {editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor">
            { render(&element_tree.as_ref().borrow(), element_tree.as_ref().borrow().elements.root.unwrap()) }
        </div>
            </span>
            </span>
    }
}
