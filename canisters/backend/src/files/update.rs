use ic_kit::candid::candid_method;
use ic_kit::macros::update;
use serde::Serialize;
use shared::Tree;
use std::collections::HashMap;
use ic_cdk;
use crate::files::types::*;
use crate::users::types::*;
use crate::utils::{Status, UpdateResponse};
use candid::{CandidType, Deserialize};
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use shared::id::Id;
use shared::schema::{FileDirectory, FileNode, FileNodeCreate};

#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: FileNodeCreate) {
    let user = User::current();
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
    // let _= create::editor_toolbar_plugin::_create_file(&mut user_files, &username, create_file_data.directory_id, create_file_data.id, create_file_data.name, create_file_data.children);
    s! { UserFiles = user_files}
    ;
}



#[update]
#[candid_method(update)]
pub async fn create_directory() -> UpdateResponse {
    let id = Id::new();
    let current_user = User::current();
    let mut user_files: UserFiles = s!(UserFiles);
    let mut file_directory = FileDirectory::new(id, "default".to_string());
    let id = Id::new();
    file_directory.files.push_vertex(
        id.into(),
        FileNode {
            id: id.into(),
            name: "root".into(),
            element_tree: None,
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
    return UpdateResponse { status: Status::Success, message: "Directory created".to_string() };
}


