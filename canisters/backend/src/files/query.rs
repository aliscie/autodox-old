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
pub fn get_file(data: String) -> Option<FileNode> {
    // TODO we should return the FileNode Or the ElementTree
    let (auther_id, file_id) = serde_json::from_str::<(Id, Id)>(&data).unwrap();
    let current_user = User::current()?;
    let mut users: Users = s!(Users);
    let mut auther: Option<User> = None;
    for u in users {
        if u.address.to_string() == auther_id.to_string() {
            auther = Some(u);
        };
    };

    let mut user_files: UserFiles = s!(UserFiles);
    let file = user_files
        .get(&auther.clone().unwrap())?
        .files
        .vertices
        .get(&file_id)
        .map(|s| s.clone());
    if &current_user == &auther.unwrap()
    // || file.file_mode == FileMode::Public
    {
        return file;
    };
    return None;
}
