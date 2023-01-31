use crate::users::types::Users;
use crate::users::types::{User, UserFiles};
use ic_kit::candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use shared::{id::Id, schema::*};

#[query]
#[candid_method(query)]
pub fn get_directories() -> Option<FileDirectory> {
    let user = User::current()?;
    let mut user_files: UserFiles = s!(UserFiles);
    user_files.get(&user).map(|s| s.clone())
}

#[query]
#[candid_method(query)]
pub fn get_file(file_id: String) -> Option<FileNode> {
    let file_id = Id::try_from(file_id).ok()?;
    let user = User::current()?;
    let mut user_files: UserFiles = s!(UserFiles);
    user_files
        .get(&user)?
        .files
        .vertices
        .get(&file_id)
        .map(|s| s.clone())
}
