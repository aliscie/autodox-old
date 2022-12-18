use std::collections::HashMap;
use ic_kit::{
    candid::candid_method,
    macros::*,
};

use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types::*;
use candid::CandidType;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::id::Id;
use shared::schema::{FileDirectory, FileNode, FileNodeCreate};
use crate::utils::{get_username, Status, UpdateResponse};
use crate::users::types::*;


#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: FileNodeCreate) -> Status {
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return Status::UnAuthorized,
        Some(username) => username
    };
    let mut user_files = s!(UserFiles);
    // let _= create::utils::_create_file(&mut user_files, &username, create_file_data.directory_id, create_file_data.id, create_file_data.name, create_file_data.children);
    s!{ UserFiles = user_files};
    Status::Success
}


#[update]
#[candid_method(update)]
pub fn create_directory(create_file_data: FileDirectory) -> Status {
    let files = create_file_data.files;
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return Status::UnAuthorized,
        Some(username) => username
    };
    let mut user_files = s!(UserFiles);
    // user_files.push(HashMap::new(username, new_file_tree));
    // let res = _create_directory(&mut user_files, &username, create_file_data.id, create_file_data.name);
    // println!("{:#?}", res);
    s! { UserFiles = user_files}
    ;
    Status::Success
}

