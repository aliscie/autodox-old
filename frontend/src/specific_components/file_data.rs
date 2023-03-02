use crate::backend::create_element;
use crate::backend::create_element_tree;
use crate::backend::delete_element;
use crate::backend::get_element_tree;
// use crate::backend::update_element;
use editor::Editor;
use shared::id::Id;
use shared::log;
use shared::schema::EditorChange;
use shared::schema::{EditorElement, ElementTree, FileDirectory, FileNode, UserQuery};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

use web_sys::{window, Range};

use yew::prelude::*;
use yew::suspense::use_future_with_deps;
use yew::suspense::SuspensionResult;
use yew::suspense::UseFutureHandle;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Id,
    pub auther: String,
}

fn onchange_element_tree(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    let changes = Dispatch::<UseChangeHandle>::new();
    Callback::from(move |e: EditorChange| {
        changes.reduce_mut(|s| s.changes.push_back(e.clone()));
        match e {
            EditorChange::Update(x) => {
                // let update_data = x.clone();
                // spawn_local(async move {
                //     let x = update_element(update_data).await;
                // });
                if let Some(element) = element_tree
                    .as_ref()
                    .borrow_mut()
                    .elements
                    .vertices
                    .get_mut(&x.id)
                {
                    if let Some(text) = x.content {
                        element.content = text;
                    }
                    if let Some(attrs) = x.attrs {
                        element.attrs = attrs;
                    }
                }
            }
            EditorChange::Create(x) => {
                let create_data = x.clone();
                spawn_local(async move {
                    let result = create_element(create_data).await;
                });
                element_tree.clone().borrow_mut().elements.push_children(
                    x.parent_id.clone(),
                    x.id.clone(),
                    x.clone().into(),
                );
                if let Some(prev_element_id) = x.prev_element_id {
                    let mut element_tree = element_tree.as_ref().borrow_mut();
                    let children_list_of_parent_element = element_tree
                        .elements
                        .adjacency
                        .get_mut(&x.parent_id)
                        .unwrap();
                    let index_of_prev_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == x.id)
                        .unwrap();
                    let index_of_last_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == x.id)
                        .unwrap();
                    children_list_of_parent_element
                        .swap(index_of_last_element, index_of_prev_element);
                }
            }
            EditorChange::Delete(data) => {
                element_tree.as_ref().borrow_mut().elements.remove(&data.id);
                spawn_local(async move {
                    let result = delete_element(data).await;
                });
            }
        };
    })
}

use crate::backend;

#[function_component]
pub fn FileData(props: &Props) -> HtmlResult {
    let res = use_element_tree(props.id)?;

    let result_html = match &*res {
        Ok((file_node, ref tree)) => {
            // log!(&tree);
            // let file_node = Dispatch::<FileDirectory>::new()
            //     .get()
            //     .files
            //     .vertices
            //     .get(&props.id)
            //     .unwrap()
            //     .clone();
            let element_tree = Rc::new(RefCell::new(tree.clone()));

            html! {
                <Editor
                title = { file_node.name.clone() }
                element_tree = { element_tree.clone() }
                onchange = { onchange_element_tree(element_tree.clone())}
               >
                </Editor>
            }
        }
        Err(ref failure) => failure.to_string().into(),
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
fn use_element_tree(file_id: Id) -> SuspensionResult<UseFutureHandle<Result<(FileNode, ElementTree), String>>> {
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    let dispatch_element_tree = Dispatch::<crate::hooks::ElementTreeStore>::new();

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
                            let file_node = dispatch_file_directory
                                .get()
                                .files
                                .vertices
                                .get(&file_id)
                                .unwrap()
                                .clone();
                            return Ok((file_node, get_element_tree(&tree_id).await.unwrap()));
                        }
                        None => {
                            // let element_tree = dummy_data();
                            // let _ = backend::create_element_tree(&element_tree, *file_id).await?;
                            // let tree_id = element_tree.id;
                            let file_node = dispatch_file_directory
                                .get()
                                .files
                                .vertices
                                .get(&file_id)
                                .unwrap()
                                .clone();
                            let dummy_element_tree = dummy_data();
                            let element_tree = dispatch_element_tree.get().map.get(&file_node.id).unwrap_or(&dummy_element_tree).clone();
                            if dispatch_element_tree.get().map.get(&file_node.id).is_none() {
                                // TODO enable people to share files also get element instead of using `dummy_data()`
                                // let _ = backend::create_element_tree(&element_tree, *file_id).await?;
                                // dispatch_element_tree.dispatch(ElementTreeStoreAction::AddElementTree(file_node.id, element_tree.clone()));
                            }
                            return Ok((file_node, element_tree));
                        }
                    };
                }
                None => {
                    let doc = window().unwrap_throw().document().unwrap_throw();
                    let editor = doc
                        .query_selector(".main_container")
                        .unwrap_throw()
                        .unwrap_throw();
                    editor.class_list().add_1("loading");
                    let url = window().unwrap_throw().location();
                    let auther = url.pathname().unwrap_throw();
                    let data = auther.split("/").collect::<Vec<&str>>();
                    let auther = data[3];
                    let data = serde_json::json!((auther, file_id.clone()));
                    let res = backend::call_ic("get_file".to_string(), data.to_string()).await;
                    log!(&res);
                    let file_dir: Result<Result<(FileNode, ElementTree), String>, _> = serde_wasm_bindgen::from_value(res);
                    if let Ok(file_dir) = file_dir {
                        if let Ok((file_node, element_tree)) = file_dir {
                            return Ok((file_node, element_tree));
                        }
                    }
                }
            }
            return Err(String::from("Not found!"));
        },
        file_id,
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseChangeHandle {
    pub changes: VecDeque<EditorChange>,
}

impl Store for UseChangeHandle {
    fn new() -> Self {
        Self {
            changes: VecDeque::new(),
        }
    }
    fn should_notify(&self, old: &Self) -> bool {
        old != self
    }
}
