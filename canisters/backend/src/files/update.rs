use ic_kit::{candid::candid_method, macros::*};
use std::collections::HashMap;

use crate::files::types::*;
use crate::users::types::*;
use crate::utils::{get_username, Status, UpdateResponse};
use candid::CandidType;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use shared::id::Id;
use shared::schema::{FileDirectory, FileNode, FileNodeCreate};

#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: FileNodeCreate) -> Status {
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return Status::UnAuthorized,
        Some(username) => username,
    };
    let mut user_files: UserFiles = s!(UserFiles);
    if let Some(file_directory) = user_files.get_mut(username) {
        let mut parent_adjacency = file_directory
            .adjacency
            .entry(create_file_data.parent_id)
            .or_default();
        parent_adjacency.insert(create_file_data.id);
        file_directory
            .vertices
            .insert(create_file_data.id, create_file_data.into());
    }
    // let _= create::utils::_create_file(&mut user_files, &username, create_file_data.directory_id, create_file_data.id, create_file_data.name, create_file_data.children);
    s! { UserFiles = user_files};
    Status::Success
}

#[update]
#[candid_method(update)]
pub fn create_directory(create_file_data: FileDirectory) -> Status {
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return Status::UnAuthorized,
        Some(username) => username,
    };
    let mut user_files = s!(UserFiles);
    user_files.insert(username, create_file_data);
    // let res = _create_directory(&mut user_files, &username, create_file_data.id, create_file_data.name);
    // println!("{:#?}", res);
    s! { UserFiles = user_files};
    Status::Success
}
