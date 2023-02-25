use crate::handle_mutation::{handle_mutation, mutate_tree};
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
    pub element_tree: ElementTree,
    pub onchange: Callback<EditorChange>,
}

pub struct Editor {
    element_tree: ElementTree,
    editor_ref: NodeRef,
    observer: Option<MutationObserver>,
    oninput_event: Option<Closure<dyn FnMut(Vec<MutationRecord>, MutationObserver)>>,
}

pub enum EditorMsg {
    Mutation(Vec<MutationRecord>),
    EditorChange(EditorChange),
}

impl Component for Editor {
    type Message = EditorMsg;
    type Properties = EditorProps;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            editor_ref: NodeRef::default(),
            observer: None,
            oninput_event: None,
            element_tree: ctx.props().element_tree.clone(),
        }
    }
    // we are getting mutation from the browser so no need to rerender then
    // but EditorMsg::EditorChange is from code so we rerender on that
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::Mutation(mutation) => {
                handle_mutation(&mutation, &ctx.props().onchange, &mut self.element_tree);
                false
            }
            EditorMsg::EditorChange(change) => {
                mutate_tree(&mut self.element_tree, &change);
                ctx.props().onchange.emit(change);
                // rerender
                true
            }
        }
    }
    fn destroy(&mut self, ctx: &Context<Self>) {
        // cleaning observer and input_event
        if let Some(observer) = &self.observer {
            observer.disconnect();
        }
        if let Some(oninput_event) = &self.oninput_event {
            drop(oninput_event);
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // we only need to run setup mutation observer once
        if !first_render {
            return;
        }
        let callback = ctx
            .link()
            .callback(|mutation_records: Vec<MutationRecord>| {
                EditorMsg::Mutation(mutation_records)
            });
        self.oninput_event = Some({
            Closure::wrap(Box::new(
                move |mutation_event: Vec<MutationRecord>, _observer: MutationObserver| {
                    callback.emit(mutation_event);
                },
            ) as Box<dyn FnMut(_, _)>)
        });
        let mutation_observer = MutationObserver::new(
            self.oninput_event
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap();
        mutation_observer.observe_with_options(
            &self.editor_ref.get().unwrap(),
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
        self.observer = Some(mutation_observer);
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let mention_clouser = |event: DropDownItem, range: Option<Range>| {};
        let emojis_command = |event: DropDownItem, range: Option<Range>| {
            // let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node(&event.value));
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
            let _ = html_document
                .exec_command_with_show_ui_and_value("InsertText", false, &event.value)
                .unwrap();
        };
        let slash_command = {
            //let element_tree = element_tree.clone();
            Callback::from(move |(event, range)| {
                //on_slash_input(event, range, element_tree.clone());
            })
        };
        html! {
            <span
                class={css_file_macro!("main.css")}
           >
                <h2 contenteditable="true" class={"editor_title heading"}>
                {ctx.props().title.clone()}
            </h2>
                <span>
                <EditorToolbar
                    editor_ref = { self.editor_ref.clone()}
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
                <div  ref =  {&self.editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor">
                { render(&self.element_tree, self.element_tree.elements.root.unwrap()) }
            </div>
                </span>
                </span>
        }
    }
}

// this is used for the work space

//#[function_component]
//pub fn Editor_Old(props: &EditorProps) -> Html {
//    let element_tree = use_state_eq(|| props.element_tree.clone());
//    // get mouse position and sort it in yewdux
//    // each time the mouse move sort the pagex and pagey again
//
//    // get current hovered element and sort it yewdux
//    // get the previous  hovered and sorted it in yewdux
//
//    // get the current focused and sorted it
//    // get the previous  focused and sorted it in yewdux
//
//    //
//    // let state = use_state(|| "".to_string());
//    let editor_ref = NodeRef::default();
//    let oninput_event = {
//        let element_tree = element_tree.clone();
//        let onchange = props.onchange.clone();
//        Closure::wrap(Box::new(
//            move |mutation_event: Vec<MutationRecord>, _observer: MutationObserver| {
//                handle_mutation(&mutation_event, &onchange, element_tree.clone());
//            },
//        ) as Box<dyn FnMut(_, _)>)
//    };
//
//    use_effect_with_deps(
//        move |editor_ref| {
//            let mutation_observer =
//                MutationObserver::new(oninput_event.as_ref().unchecked_ref()).unwrap();
//            //let doc = window().unwrap_throw().document().unwrap_throw();
//            //let editor: Rc<Element> = Rc::new(editor_ref.c!(ast::<Element>().unwrap());
//
//            // TODO
//            //  nested update is a problem
//            //  If we create a nested element
//            //   It is coming as update on the parent element.
//            //   It should show as a create event on the root
//
//            let _ = mutation_observer.observe_with_options(
//                &editor_ref.get().unwrap(),
//                MutationObserverInit::new()
//                    // child attributes or editor attributes chanding
//                    .attributes(true)
//                    // a new child get created or deleted
//                    .child_list(true)
//                    // user typed something
//                    .character_data(true)
//                    .character_data_old_value(true)
//                    .subtree(true),
//            );
//            //PasteConverter::new(editor.clone());
//            //TODO
//            // DragAndDrop::new(editor.clone());
//            // Mention::new(editor.clone(), reg_ex("@\w+"), mentions_components_list); // use the mention plugin to insert mention inline specific_components
//            // Mention::new(editor.clone(), "\//w+", components_list); // use the mention plugin for / insert component blocks
//            // Mention::new(editor.clone(), "\:/w+",emojis_components_list); // use the mention plugin for : insert emojis inline
//
//            move || {
//                drop(oninput_event);
//                mutation_observer.disconnect();
//            }
//        },
//        editor_ref.clone(),
//    );
//
//    let element_tree = element_tree.clone();
//
//    let onkeydown: Callback<KeyboardEvent> = Callback::from(move |_e: KeyboardEvent| {
//        if _e.key() == "Tab" {
//            _e.prevent_default();
//            let window = web_sys::window().unwrap();
//            let document = window.document().unwrap();
//            let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
//            let _ = html_document
//                .exec_command_with_show_ui_and_value("InsertText", false, "    ")
//                .unwrap();
//        }
//    });
//    // TODO make the commands Callback<DropDownItem, Option<Range>> instead of fn(DropDownItem, Option<Range>)
//    let emojis_command: fn(DropDownItem, Option<Range>) = (|event, range| {
//        // let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node(&event.value));
//        let window = web_sys::window().unwrap();
//        let document = window.document().unwrap();
//        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
//        let _ = html_document
//            .exec_command_with_show_ui_and_value("InsertText", false, &event.value)
//            .unwrap();
//    });
//    let slash_command = {
//        let element_tree = element_tree.clone();
//        Callback::from(move |(event, range)| {
//            on_slash_input(event, range, element_tree.clone());
//        })
//    };
//    let action: Callback<String> = Callback::from(move |e: String| {
//        // log!(e.clone());
//        // onchange.emit(EditorChange::Update(EditorElementUpdate {
//        //     id: element_tree.elements.root.unwrap(),
//        //     text_format: Some(format),
//        //     ..Default::default()
//        // }));
//    });
//
//    let mention_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {});
//
//    // let format_command: fn(String, selectoin) -> Option<()> =  (|event, range| return Some((
//    //     onchange.emit(EditorChange::Update(update)); // TODO this should be the same for  on_slash_input, mention_clouser and emojis_command
//    //     )));
//
//    html! {
//        <span
//            class={css_file_macro!("main.css")}
//       >
//            <h2 contenteditable="true" class={"editor_title heading"}>
//            {props.title.clone()}
//        </h2>
//            <span>
//            <EditorToolbar
//                editor_ref = { editor_ref.clone()}
//            // command={Callback::from(move |(e, r)| format_command(e, r))}
//            />
//            <EditorInsert
//                items={insertion_closures::components()}
//                trigger={"/".to_string()}
//                command={slash_command}/>
//            <EditorInsert
//                items={insertion_closures::mentions()}
//                trigger={"@".to_string()}
//                command={Callback::from(move |(e, r)| mention_clouser(e, r))}/>
//            <EditorInsert
//                items={insertion_closures::emojies()}
//                trigger={":".to_string()}
//                command={Callback::from(move |(e, r) | emojis_command(e, r))}/>
//            <div  ref =  {editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor">
//            { render(&element_tree, element_tree.elements.root.unwrap()) }
//        </div>
//            </span>
//            </span>
//    }
//}
