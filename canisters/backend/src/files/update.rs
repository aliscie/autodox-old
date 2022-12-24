use ic_kit::{candid::candid_method};
use ic_kit::macros::update;
use serde::Serialize;
use shared::Tree;
use std::collections::HashMap;

use crate::files::types::*;
use crate::users::types::*;
use crate::utils::{get_username, Status, UpdateResponse};
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
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return (),
        Some(username) => username,
    };
    let mut user_files: UserFiles = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(&username) {
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

use ic_cdk::export::Principal;
use ic_cdk;

#[update]
#[candid_method(update)]
pub async fn create_directory(create_file_data: FileDirectory) {
    let id: Result<(Vec<u8>, ), _> = ic_cdk::api::call::call(
        ic_cdk::export::Principal::management_canister(),
        "raw_rand",
        (),
    ).await;

    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return (),
        Some(username) => username,
    };
    let mut user_files: UserFiles = s!(UserFiles);
    let file_directory = FileDirectory {
        id: id.unwrap().into(),
        name: create_file_data.name,
        files: create_file_data.files,
    };
    user_files.insert(username, file_directory);
    s! { UserFiles = user_files}
    ;
}

#[update]
#[candid_method(update)]
pub fn test(data: String) -> Status {
    Status::Success
}
