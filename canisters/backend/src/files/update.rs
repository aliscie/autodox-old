use std::collections::HashMap;
use std::collections::VecDeque;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk;
use ic_kit::candid::candid_method;
use ic_kit::macros::update;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use indexmap::IndexSet;
use serde::Serialize;

use shared::id::Id;
use shared::schema::{EditorChange, EditorElement, ElementTree, FileDirectory, FileMode, FileNode, FileNodeCreate, FileNodeDelete, FileNodeMove, FileNodeUpdate};
use shared::Tree;

use crate::elements::types::ElementTrees;
use crate::files::types::*;
use crate::users::types::*;
use crate::utils::{Status, UpdateResponse};

async fn dummy_data() -> ElementTree {
    let mut tree = Tree::new();
    let element = EditorElement {
        id: Id::ic_new().await.into(),
        tag: None,
        content: "".to_owned(),
        attrs: HashMap::new(),
    };
    tree.root = Some(element.id);
    tree.vertices.insert(element.id, element);
    let mut default_element_tree = ElementTree {
        id: Id::ic_new().await.into(),
        elements: tree,
    };
    let root_id = default_element_tree.elements.root.unwrap();
    let id: Id = Id::ic_new().await.into();
    ic_cdk::println!("---------------------------- {:#?}", &id);

    default_element_tree.elements.push_children(
        root_id,
        id.clone(),
        EditorElement::new(
            id,
            "bold text".to_string(),
            HashMap::from([("style".to_string(), "font-weight: bold;".to_string())]),
        ),
    );
    let id: Id = Id::ic_new().await.into();
    default_element_tree.elements.push_children(
        root_id,
        id,
        EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
    );
    return default_element_tree;
}


// async fn dummy_data() -> ElementTree {
//     let mut default_element_tree = ElementTree{
//         id: Ic::ic_new().await,
//         elements: Tree{
//             vertices: (),
//             adjacency: (),
//             root: None,
//         },
//     };
//     let root_id = default_element_tree.elements.root.unwrap();
//     let id: Id = Id::ic_new().await;;
//     default_element_tree.elements.push_children(
//         root_id,
//         id.clone(),
//         EditorElement::new(
//             id,
//             "bold text".to_string(),
//             HashMap::from([("style".to_string(), "font-weight: bold;".to_string())]),
//         ),
//     );
//     let id: Id = Id::ic_new().await;
//     default_element_tree.elements.push_children(
//         root_id,
//         id,
//         EditorElement::new(id, r#"Element is here."#.to_string(), HashMap::new()),
//     );
//     return default_element_tree;
// }


#[update]
#[candid_method(update)]
pub async fn create_file(data: String) -> String {
    let mut create_file_data = serde_json::from_str::<FileNodeCreate>(&data).unwrap();    let user = User::current();
    if user.is_none() {
        return "user not found".to_string();
    };
    let mut user_files: UserFiles = s!(UserFiles);

    let mut element_trees: ElementTrees = s!(ElementTrees);
    let new_element_tree = dummy_data().await;
    create_file_data.element_tree = Some(new_element_tree.id.clone());
    let new_element_tree = HashMap::from_iter(vec![(create_file_data.id.clone(), new_element_tree)]);
    element_trees.insert(user.clone().unwrap(), new_element_tree);

    if let Some(file_directory) = user_files.get_mut(&user.unwrap()) {
        let mut parent_adjacency = file_directory
            .files
            .adjacency
            .entry(create_file_data.parent_id)
            .or_default();
        parent_adjacency.push(create_file_data.id);
        file_directory
            .files
            .vertices
            .insert(create_file_data.id.clone(), create_file_data.into());
    }

    // TODO creat files here.
    // element_trees.insert(user.clone().unwrap(), HashMap::from([(create_file_data.id, dummy_data().await)]));
    s! { UserFiles = user_files}
    ;
    s! { ElementTrees = element_trees}
    ;
    return "New file is created.".to_string();
}

#[update]
#[candid_method(update)]
pub fn update_file(data: String) -> String {
    let file_node = serde_json::from_str::<FileNode>(&data).unwrap();
    let user = User::current();
    if user.is_none() {
        return "user not found".to_string();
    };
    let mut user_files: UserFiles = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(&user.unwrap()) {
        file_directory
            .files
            .vertices
            .insert(file_node.id, file_node.into());
    }
    s! { UserFiles = user_files}
    ;
    "file is updated.".to_string()
}

#[update]
#[candid_method(update)]
pub fn delete_file(json_data: String) -> String {
    let data = serde_json::from_str::<FileNodeDelete>(&json_data).unwrap();
    let user = User::current();
    if user.is_none() {
        return "user not found".to_string();
    };
    let mut user_files: UserFiles = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(&user.unwrap()) {
        let adjacency = file_directory
            .files
            .adjacency
            .get_mut(&data.parent_id)
            .unwrap();
        if adjacency.len() > 0 {
            let index = adjacency.iter().position(|x| *x == data.id).unwrap();
            adjacency.remove(index);
        }
        file_directory.files.vertices.remove(&data.id);
    }
    s! {UserFiles = user_files}
    ;
    "File is deleted.".to_string()
}

#[update]
#[candid_method(update)]
pub async fn group_update(data: String) -> Option<String> {
    let changes = serde_json::from_str::<Vec<EditorChange>>(&data).ok()?;
    let user = User::current()?;
    let mut user_files = s!(UserFiles);
    let file_directory: &FileDirectory = user_files.get(&user)?;
    let mut element_trees: ElementTrees = s!(ElementTrees);
    for change in changes {
        match change {
            EditorChange::Create(data) => {
                let element_tree: &mut ElementTree = element_trees.get_mut(&user)?.get_mut(&data.tree_id)?;
                element_tree.elements.push_children(
                    data.parent_id.clone(),
                    data.id.clone(),
                    data.clone().into(),
                );
                if let Some(prev_element_id) = data.prev_element_id {
                    let children_list_of_parent_element =
                        element_tree.elements.adjacency.get_mut(&data.parent_id)?;
                    let index_of_prev_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == data.id)?;
                    let index_of_last_element = children_list_of_parent_element
                        .into_iter()
                        .position(|y| *y == data.id)?;
                    children_list_of_parent_element
                        .swap(index_of_last_element, index_of_prev_element);
                }
            }
            EditorChange::Update(data) => {
                let element_tree: &mut ElementTree = element_trees.get_mut(&user)?.get_mut(&data.tree_id)?;
                if let Some(element) = element_tree.elements.vertices.get_mut(&data.id) {
                    if let Some(text) = data.content {
                        element.content = text;
                    }
                    if let Some(attrs) = data.attrs {
                        element.attrs = attrs;
                    }
                }
            }
            EditorChange::Delete(data) => {
                let element_tree: &mut ElementTree = element_trees.get_mut(&user)?.get_mut(&data.tree_id)?;
                element_tree.elements.remove(&data.id);
            } // EditorChange::MoveFile(data) => {
            //     let adjacency = file_directory
            //         .files
            //         .adjacency
            //         .get_mut(&data.parent_id)
            //         .unwrap();
            //     if adjacency.len() > 0 {
            //         let index = adjacency.iter().position(|x| *x == data.id).unwrap();
            //         adjacency.remove(index);
            //     }
            //     let mut parent_adjacency = file_directory
            //         .files
            //         .adjacency
            //         .entry(data.new_parent_id)
            //         .or_default();
            //     parent_adjacency.push(data.id);
            // }
        }
    }
    s! { UserFiles = user_files}
    ;
    Some("Files are updated.".to_string())
}

#[update]
#[candid_method(update)]
pub async fn create_directory() -> String {
    let current_user = User::current();

    if current_user.clone().is_none() {
        return "Anonymous users cannot create directories".to_string();
    };

    let id: Id = Id::ic_new().await;

    let mut user_files: UserFiles = s!(UserFiles);
    if !user_files.get(&current_user.clone().unwrap()).is_none() {
        return "User already have directory.".to_string();
    };

    let mut file_directory = FileDirectory::new(id, "default".to_string());
    let id: Id = Id::ic_new().await;
    file_directory.files.push_vertex(
        id.into(),
        FileNode {
            id: id.into(),
            name: "root".into(),
            element_tree: None,
            file_mode: FileMode::Private,
            // users_can_see: [].to_vec(),
        },
    );
    file_directory
        .files
        .adjacency
        .insert(id.clone().into(), Vec::new());
    file_directory.files.root = Some(id.into());
    user_files.insert(current_user.unwrap(), file_directory.clone());
    s! { UserFiles = user_files}
    ;
    "New directory is created.".to_string()
}

#[update]
#[candid_method(update)]
pub async fn rename_file(data: String) -> String {
    let json_data = serde_json::from_str::<FileNodeUpdate>(&data).unwrap();
    let user = User::current().unwrap();
    let mut user_files = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(&user) {
        file_directory
            .files
            .vertices
            .get_mut(&json_data.id)
            .unwrap()
            .name = json_data.name.unwrap().clone();
        s! { UserFiles = user_files}
        ;
    };
    return "File is renamed".to_string();
}

#[update]
#[candid_method(update)]
pub async fn change_directory(data: String) -> String {
    let json_data = serde_json::from_str::<FileNodeMove>(&data).unwrap();
    let user = User::current().unwrap();
    let mut user_files = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(&user) {
        let old_adjacency = file_directory
            .files
            .adjacency
            .get_mut(&json_data.old_parent_id)
            .unwrap();
        if old_adjacency.len() > 0 {
            let file_index = old_adjacency
                .iter()
                .position(|x| *x == json_data.id)
                .unwrap();
            if file_index >= 0 {
                old_adjacency.remove(file_index);
            }
        }
        let mut new_adjacency = file_directory
            .files
            .adjacency
            .entry(json_data.new_parent_id)
            .or_default();
        new_adjacency.push(json_data.id);
        s! {UserFiles = user_files}
        ;
    };
    return "File is moved".to_string();
}
