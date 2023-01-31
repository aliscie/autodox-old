// editor/src/insertion_component
extern crate web_sys;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, Range, window};
use yew::*;

use app::Editor;
pub use dummy_data::*;
use shared::id::Id;
use shared::schema::{
    EditorElement, EditorElementCreate, EditorElementUpdate, ElementTree, FileNode,
};
use shared::tree::Tree;

use crate::app::EditorChange;

mod editor_components;

mod insertion_closures;
mod app;
mod backend;
pub(crate) mod components;
mod plugins;
mod render;
pub(crate) mod spesific_components;
mod dummy_data;

use shared::*;


use shared::*;
use wasm_bindgen::{JsCast};
use web_sys::{MutationObserverInit, MutationRecord, Node};
use yew::{function_component, html};


fn onchange(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        // log!(&e)
    })
}

use crate::plugins::{EditorToolbar, EditorInsert, CommandItems, DropDownItem};

#[function_component]
pub fn App() -> Html {
    let element_tree = generate_dummy();

    let action: Callback<String> = Callback::from(move |e: String| {
        // log!(e.clone());
        // onchange.emit(EditorChange::Update(EditorElementUpdate {
        //     id: element_tree.as_ref().borrow().elements.root.unwrap(),
        //     text_format: Some(format),
        //     ..Default::default()
        // }));
    });
    let slash_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {
        let text = event.text.as_str();
        match text {
            "table" => {
                let table = r#"<table><tr> <th>Company</th> <th>Contact</th> <th>Country</th> </tr> <tr> <td>Alfreds Futterkiste</td> <td>Maria Anders</td> <td>Germany</td> </tr> <tr> <td>Centro comercial Moctezuma</td> <td>Francisco Chang</td> <td>Mexico</td> </tr> <tr> <td>Ernst Handel</td> <td>Roland Mendel</td> <td>Austria</td> </tr> <tr> <td>Island Trading</td> <td>Helen Bennett</td> <td>UK</td> </tr> <tr> <td>Laughing Bacchus Winecellars</td> <td>Yoshi Tannamuri</td> <td>Canada</td> </tr> <tr> <td>Magazzini Alimentari Riuniti</td> <td>Giovanni Rovelli</td> <td>Italy</td> </tr></table>"#;
                let mut table_element = window().unwrap_throw().document().unwrap_throw().create_element("table").unwrap_throw();
                table_element.set_inner_html(table.replace('\n', "").as_str());
                let _ = range.unwrap_throw().insert_node(&table_element);
            }
            "image" => {}
            _ => {}
        };
    });
// TODO make the commands Callback<DropDownItem, Option<Range>> instead of fn(DropDownItem, Option<Range>)
    let emojis_command: fn(DropDownItem, Option<Range>) = (|event, range| {
// let _ = range.unwrap().insert_node(&window().unwrap_throw().document().unwrap_throw().create_text_node(&event.value));
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let _ = html_document.exec_command_with_show_ui_and_value("InsertText", false, &event.value).unwrap();
    });

    let mention_clouser: fn(DropDownItem, Option<Range>) = (|event, range| {
        log!(event.value);
    });

    html! {
       <div>
       <Editor
           title = {"untitled".to_string()}
           element_tree={element_tree.clone()}
           onchange = { onchange(element_tree.clone())}
      >
           <EditorToolbar  action={action}/>
            <EditorInsert items={insertion_closures::components()}  trigger={"/".to_string()} command={slash_clouser}/>
            <EditorInsert items={insertion_closures::mentions()}  trigger={"@".to_string()} command={mention_clouser}/>
            <EditorInsert items={insertion_closures::emojies()}  trigger={":".to_string()}  command={emojis_command}/>
        </Editor>
       </div>
   }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


