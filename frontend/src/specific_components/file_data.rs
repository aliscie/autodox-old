use crate::backend::create_element;
use crate::backend::create_element_tree;
use crate::backend::get_element_tree;
use crate::backend::update_element;
use editor::Editor;
use editor::EditorChange;
use shared::id::Id;
use shared::log;
use shared::schema::{Attrs, EditorElement, ElementTree, FileDirectory};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
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
                element_tree.as_ref().borrow_mut().elements.push_children(
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
            _ => {}
        };
    })
}

#[function_component(FileData)]
pub fn file_data(props: &Props) -> HtmlResult {
    let res = use_element_tree(props.id)?;
    let result_html = match *res {
        Ok(ref tree) => {
            let file_node = Dispatch::<FileDirectory>::new()
                .get()
                .files
                .vertices
                .get(&props.id)
                .unwrap()
                .clone();
            let element_tree = Rc::new(RefCell::new(tree.clone()));
            html! {
                <Editor
                    title = { file_node.name.clone() }
                element_tree = { element_tree.clone() }
                onchange = { onchange_element_tree(element_tree.clone())}
                />
            }
        }
        Err(ref failure) => {
            log!(failure);
            failure.to_string().into()
        }
    };
    Ok(result_html)
}

#[hook]
fn use_element_tree(file_id: Id) -> SuspensionResult<UseFutureHandle<Result<ElementTree, String>>> {
    let dispatch = Dispatch::<FileDirectory>::new();
    use_future_with_deps(
        |file_id| async move {
            match dispatch.get().files.vertices.get(&file_id) {
                Some(current_file_data) => {
                    match current_file_data.element_tree {
                        Some(tree_id) => {
                            return get_element_tree(&tree_id).await;
                        }
                        None => {
                            // create new element_tree
                            let mut default_element_tree = ElementTree::default();
                            let root = default_element_tree.elements.root.unwrap();
                            let id: Id = Uuid::new_v4().into();
                            default_element_tree.elements.push_children(
                                root,
                                id.clone(),
                                EditorElement::new(
                                    id,
                                    "bold text".to_string(),
                                    HashMap::from([(
                                        Attrs::Style,
                                        "font-weight: bold;".to_string(),
                                    )]),
                                ),
                            );
                            let id: Id = Uuid::new_v4().into();
                            default_element_tree.elements.push_children(
                                root,
                                id,
                                EditorElement::new(
                                    id,
                                    r#"Element is here."#.to_string(),
                                    HashMap::new(),
                                ),
                            );
                            let _ = create_element_tree(&default_element_tree, *file_id).await?;
                            let tree_id = default_element_tree.id;
                            dispatch.reduce_mut(|f| {
                                let file_node = f.files.vertices.get_mut(&file_id).unwrap();
                                file_node.id = tree_id;
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
