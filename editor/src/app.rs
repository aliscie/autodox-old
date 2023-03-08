use crate::editor_components::EditorElementProps;
use crate::handle_mutation::{handle_mutation, mutate_tree};
use crate::insertion_closures;
use crate::plugins::{
    CommandItems, ContextMenu, DropDownItem, EditorInsert, EditorToolbar, Position,
};
use crate::render::render;
use crate::utils::on_slash_input;
use serde::{Deserialize, Serialize};
use shared::id::Id;
use shared::schema::{
    EditorChange, EditorElement, EditorElementCreate, EditorElementDelete, EditorElementUpdate,
    ElementTree,
};
use shared::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    window, Element, MutationObserver, MutationObserverInit, MutationRecord, Node, Range,
};
use yew::html::{HasAllProps, HasProp, IntoPropValue};
use yew::prelude::*;
use yew::{function_component, html};

#[derive(PartialEq, Clone)]
pub struct GlobalEditorState {
    pub element_tree: Rc<ElementTree>,
    pub mutation: Callback<Vec<EditorChange>>,
    pub render_context_menu: Callback<(MouseEvent, Html)>,
}

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub title: String,
    pub element_tree: Rc<ElementTree>,
    pub onchange: Callback<EditorChange>,
}

pub struct Editor<T>
where
    T: BaseComponent + BaseComponent<Properties = EditorElementProps>,
{
    element_tree: Rc<ElementTree>,
    editor_ref: NodeRef,
    observer: Option<MutationObserver>,
    oninput_event: Option<Closure<dyn FnMut(Vec<MutationRecord>, MutationObserver)>>,
    context_menu_position: Option<Position>,
    context_menu_items: Html,
    _element_marker: PhantomData<T>,
}

pub enum EditorMsg {
    Mutation(Vec<MutationRecord>),
    EditorChange(Vec<EditorChange>),
    SlashInput(DropDownItem, Option<Range>),
    ContextMenuRender((MouseEvent, Html)),
}

impl<T> Component for Editor<T>
where
    T: BaseComponent + BaseComponent<Properties = EditorElementProps>,
{
    type Message = EditorMsg;
    type Properties = EditorProps;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            editor_ref: NodeRef::default(),
            observer: None,
            oninput_event: None,
            element_tree: ctx.props().element_tree.clone(),
            context_menu_position: None,
            context_menu_items: html! {},
            _element_marker: PhantomData,
        }
    }
    // we are getting mutation from the browser so no need to rerender then
    // but EditorMsg::EditorChange is from code so we rerender on that
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMsg::Mutation(mutation) => {
                handle_mutation(
                    &mutation,
                    &ctx.props().onchange,
                    Rc::make_mut(&mut self.element_tree),
                );
                false
            }
            EditorMsg::EditorChange(change) => {
                for i in change {
                    mutate_tree(Rc::make_mut(&mut self.element_tree), &i);
                    ctx.props().onchange.emit(i);
                }
                // rerender
                true
            }
            EditorMsg::SlashInput(event, range) => {
                on_slash_input(event, range, Rc::make_mut(&mut self.element_tree));
                true
            }
            EditorMsg::ContextMenuRender((e, items)) => {
                e.prevent_default();
                self.context_menu_position = Some(Position {
                    x: e.x().into(),
                    y: e.y().into(),
                });
                self.context_menu_items = items;
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
        let callback = ctx
            .link()
            .callback(|change| EditorMsg::EditorChange(change));
        let render_context_menu = ctx.link().callback(|c| EditorMsg::ContextMenuRender(c));
        let global_state = GlobalEditorState {
            element_tree: self.element_tree.clone(),
            mutation: callback,
            render_context_menu,
        };
        let mention_clouser = |event: DropDownItem, range: Option<Range>| {};
        let emojis_command = |event: DropDownItem, range: Option<Range>| {
            log!(&event);
            // let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node(&event.value));
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
            let _ = html_document
                .exec_command_with_show_ui_and_value("InsertText", false, &event.value)
                .unwrap();
        };
        let slash_command = ctx
            .link()
            .callback(|(event, range)| EditorMsg::SlashInput(event, range));
        html! {
        <ContextProvider<GlobalEditorState> context = {global_state}>
            <span class="text_editor_container">
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
                    command={slash_command}
                    editor_ref = {self.editor_ref.clone()} />
                <EditorInsert
                    items={insertion_closures::mentions()}
                    trigger={"@".to_string()}
                    command={Callback::from(move |(e, r)| mention_clouser(e, r))}
                    editor_ref = {self.editor_ref.clone()}/>
                <EditorInsert
                    items={insertion_closures::emojies()}
                    trigger={":".to_string()}
                    command={Callback::from(move |(e, r) | emojis_command(e, r))}
                    editor_ref = {self.editor_ref.clone()} />
                <ContextMenu position = {self.context_menu_position.clone()}>
                    {self.context_menu_items.clone()}
                </ContextMenu>
                <div  ref =  {&self.editor_ref}  contenteditable = "true" class="text_editor" id = "text_editor"> // now we can pass different component as type
                { render::<T>(&self.element_tree, self.element_tree.elements.root.unwrap()) }
            </div>
                </span>
                </span>
            </span>
        </ContextProvider<GlobalEditorState>>
        }
    }
}
