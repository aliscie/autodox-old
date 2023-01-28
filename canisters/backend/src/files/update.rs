use std::collections::HashMap;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk;
use ic_kit::candid::candid_method;
use ic_kit::macros::update;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use serde::Serialize;

use shared::id::Id;
use shared::schema::{
    FileDirectory, FileNode, FileNodeCreate, FileNodeDelete, FileNodeMove, FileNodeUpdate,
};
use shared::Tree;

use crate::files::types::*;
use crate::users::types::*;
use crate::utils::{Status, UpdateResponse};

#[update]
#[candid_method(update)]
pub fn create_file(data: String) -> String {
    let create_file_data = serde_json::from_str::<FileNodeCreate>(&data).unwrap();
    let user = User::current();
    if user.is_none() {
        return "user not found".to_string();
    };
    let mut user_files: UserFiles = s!(UserFiles);

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
            .insert(create_file_data.id, create_file_data.into());
    }
    // let _= create::_create_file(&mut user_files, &username, create_file_data.directory_id, create_file_data.id, create_file_data.name, create_file_data.children);
    s! { UserFiles = user_files};
    "New file is created.".to_string()
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
    s! {UserFiles = user_files};
    "File is deleted.".to_string()
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
            test: None,
        },
    );
    file_directory
        .files
        .adjacency
        .insert(id.clone().into(), Vec::new());
    file_directory.files.root = Some(id.into());
    user_files.insert(current_user.unwrap(), file_directory.clone());
    s! { UserFiles = user_files};
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
        s! { UserFiles = user_files};
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
            if (file_index >= 0) {
                old_adjacency.remove(file_index);
            }
        }
        let mut new_adjacency = file_directory
            .files
            .adjacency
            .entry(json_data.new_parent_id)
            .or_default();
        new_adjacency.push(json_data.id);
        s! {UserFiles = user_files};
    };
    return "File is moved".to_string();
}
