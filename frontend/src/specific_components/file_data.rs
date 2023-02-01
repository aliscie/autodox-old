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
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

use wasm_bindgen::JsCast;
use web_sys::{window, Range};
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew::suspense::SuspensionResult;
use yew::suspense::UseFutureHandle;
use yewdux::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Id,
}

fn onchange_element_tree(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    let changes = Dispatch::<UseChangeHandle>::new();
    Callback::from(move |e: EditorChange| {
        changes.reduce_mut(|s| s.changes.push_back(e.clone()));
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
                // if let Some(prev_element_id) = x.prev_element_id {
                //     let mut element_tree = element_tree.as_ref().borrow_mut();
                //     let children_list_of_parent_element = element_tree
                //         .elements
                //         .adjacency
                //         .get_mut(&x.parent_id)
                //         .unwrap();
                //     let index_of_prev_element = children_list_of_parent_element
                //         .get_index_of(&prev_element_id)
                //         .unwrap();
                //     let index_of_last_element =
                //         children_list_of_parent_element.get_index_of(&x.id).unwrap();
                //     children_list_of_parent_element
                //         .move_index(index_of_last_element, index_of_prev_element + 1);
                //     //log!(element_tree.elements.adjacency.get(&x.parent_id));
                // }
            }
            EditorChange::Delete(data) => {
                element_tree.as_ref().borrow_mut().elements.remove(&data.id);
                spawn_local(async move {
                    let result = delete_element(data).await;
                    log!(result);
                });
            }
        };
    })
}

use editor::plugins::{CommandItems, DropDownItem, EditorInsert, EditorToolbar};

#[function_component]
pub fn FileData(props: &Props) -> HtmlResult {
    // TODO use backend::get_file here
    //  Even the hook maybe should consider different way in order to have all the if statements inside teh backend::get_file
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();

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
            let action: fn(String, Selection) = (|event: String, selection: Selection| {
                // get the id of the parent element of the selection
                log!(selection.focus_node().unwrap().parent_element().unwrap().id());


            html! {
                <Editor
                title = { file_node.name.clone() }
                element_tree = { element_tree.clone() }
                onchange = { onchange_element_tree(element_tree.clone())}
               >
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

fn dummy_data() -> ElementTree {
    let mut default_element_tree = ElementTree::default();
    let root_id = default_element_tree.elements.root.unwrap();
    let id: Id = Uuid::new_v4().into();
    default_element_tree.elements.push_children(
        root_id,
        id.clone(),
        EditorElement::new(
            id,
            "bold text".to_string(),
            HashMap::from([("style".to_string(), "font-weight: bold;".to_string())]),
        ),
    );
    let id: Id = Uuid::new_v4().into();
    default_element_tree.elements.push_children(
        root_id,
        id,
        EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
    );
    return default_element_tree;
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
                            let default_element_tree = dummy_data();
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

#[derive(Debug, Clone)]
pub struct UseChangeHandle {
    changes: VecDeque<EditorChange>,
}

impl Store for UseChangeHandle {
    fn new() -> Self {
        Self {
            changes: VecDeque::new(),
        }
    }
    fn should_notify(&self, old: &Self) -> bool {
        false
    }
}
