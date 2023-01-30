use crate::backend::create_element;
use crate::backend::create_element_tree;
use crate::backend::delete_element;
use crate::backend::get_element_tree;
use crate::backend::update_element;
use editor::Editor;
use editor::EditorChange;
use shared::id::Id;
use shared::log;
use shared::schema::EditorElementDelete;
use shared::schema::{EditorElement, ElementTree, FileDirectory};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Range, window};
use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew::suspense::SuspensionResult;
use yew::suspense::UseFutureHandle;
use yewdux::prelude::*;
use wasm_bindgen::{JsCast};
use editor::insertion_closures;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Id,
}

fn onchange_element_tree(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        match e {
            EditorChange::Update(x) => {
                log!(&x);
                let update_data = x.clone();
                spawn_local(async move {
                    let x = update_element(update_data).await;
                    log!(x);
                });
                if let Some(element) = element_tree
                    .as_ref()
                    .borrow_mut()
                    .elements
                    .vertices
                    .get_mut(&x.id)
                {
                    if let Some(text) = x.text {
                        element.text = text;
                    }
                    if let Some(attrs) = x.attrs {
                        element.attrs = attrs;
                    }
                }
            }
            EditorChange::Create(x) => {
                log!(&x);
                let create_data = x.clone();
                spawn_local(async move {
                    let result = create_element(create_data).await;
                    log!(result);
                });
                element_tree.clone().borrow_mut().elements.push_children(
                    x.parent_id.clone(),
                    x.id.clone(),
                    x.clone().into(),
                );
                //if let Some(prev_element_id) = x.prev_element_id{
                //let mut element_tree = element_tree.as_ref().borrow_mut();
                //let children_list_of_parent_element = element_tree.elements.adjacency.get_mut(&x.parent_id).unwrap();
                //let index_of_prev_element  = children_list_of_parent_element.get_index_of(&prev_element_id).unwrap();
                //let index_of_last_element =  children_list_of_parent_element.get_index_of(&x.id).unwrap();
                //children_list_of_parent_element.move_index(index_of_last_element, index_of_prev_element + 1);
                ////log!(element_tree.elements.adjacency.get(&x.parent_id));
                //}
            }
            EditorChange::Delete(id) => {
                log!(&id);
                let parent_id = element_tree.clone().borrow_mut().elements.remove(&id);
                let data = EditorElementDelete {
                    id,
                    parent_id,
                    tree_id: element_tree.clone().borrow().id,
                };
                spawn_local(async move {
                    let result = delete_element(data).await;
                    log!(result);
                });
            }
        };
    })
}

use editor::plugins::{EditorToolbar, EditorInsert, CommandItems, DropDownItem};

#[function_component]
pub fn FileData(props: &Props) -> HtmlResult {
    let res = use_element_tree(props.id)?;
    let result_html = match *res {
        Ok(ref tree) => {
            log!(&tree);
            let file_node = Dispatch::<FileDirectory>::new()
                .get()
                .files
                .vertices
                .get(&props.id)
                .unwrap()
                .clone();
            let element_tree = Rc::new(RefCell::new(tree.clone()));

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
                <Editor
                title = { file_node.name.clone() }
                element_tree = { element_tree.clone() }
                onchange = { onchange_element_tree(element_tree.clone())}
               >
                    <EditorToolbar  action={action}/>
                    <EditorInsert items={insertion_closures::components()}  trigger={"/".to_string()} command={slash_clouser}/>
                    <EditorInsert items={insertion_closures::mentions()}  trigger={"@".to_string()} command={mention_clouser}/>
                    <EditorInsert items={insertion_closures::emojies()}  trigger={":".to_string()}  command={emojis_command}/>

                </Editor>
            }
        }
        Err(ref failure) => {
            log!(failure);
            failure.to_string().into()
        }
    };
    Ok(result_html) // TODO after loading the data this should rerender
}

#[hook]
fn use_element_tree(file_id: Id) -> SuspensionResult<UseFutureHandle<Result<ElementTree, String>>> {
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    use_future_with_deps(
        |file_id| async move {
            match dispatch_file_directory
                .get()
                .files
                .vertices
                .get(&file_id)
                .to_owned()
            {
                Some(current_file_data) => {
                    log!(current_file_data);
                    match current_file_data.element_tree {
                        Some(tree_id) => {
                            return get_element_tree(&tree_id).await;
                        }
                        None => {
                            // create new element_tree
                            let mut default_element_tree = ElementTree::default();
                            let root_id = default_element_tree.elements.root.unwrap();
                            let id: Id = Uuid::new_v4().into();
                            default_element_tree.elements.push_children(
                                root_id,
                                id.clone(),
                                EditorElement::new(
                                    id,
                                    "bold text".to_string(),
                                    HashMap::from([(
                                        "style".to_string(),
                                        "font-weight: bold;".to_string(),
                                    )]),
                                ),
                            );
                            let id: Id = Uuid::new_v4().into();
                            default_element_tree.elements.push_children(
                                root_id,
                                id,
                                EditorElement::new(
                                    id,
                                    r#"Element is here."#.to_string(),
                                    HashMap::new(),
                                ),
                            );
                            // let _ = create_element_tree(&default_element_tree, *file_id).await?;
                            let tree_id = default_element_tree.id;
                            dispatch_file_directory.clone().reduce_mut(|f| {
                                let file_node = f.files.vertices.get_mut(&file_id).unwrap();
                                // file_node.element_tree = Some(Uuid::new_v4().into());
                            });
                            return Ok(default_element_tree);
                        }
                    };
                }
                None => return Err(String::from("Not found!")),
            }
        },
        file_id,
    )
}
