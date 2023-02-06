use std::collections::HashMap;
use crate::users::types::Users;
use crate::users::types::{User, UserFiles};
use ic_kit::candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use shared::{id::Id, schema::*, Tree};

#[query]
#[candid_method(query)]
pub fn get_directories() -> Option<FileDirectory> {
    let user = User::current()?;
    let mut user_files: UserFiles = s!(UserFiles);
    user_files.get(&user).map(|s| s.clone())
}

#[query]
#[candid_method(query)]
pub async fn get_directory(data: String) -> Option<FileDirectory> {
    let mut user_files: UserFiles = s!(UserFiles);
    let users: Users = s!(Users);

    let (auther_id, file_id) = serde_json::from_str::<(Id, Id)>(&data).unwrap();
    let current_user = User::current()?;

    let mut auther: Option<User> = None;
    for u in users {
        if u.address.to_string() == auther_id.to_string() {
            auther = Some(u);
        };
    };

    let file = user_files
        .get(&auther.clone().unwrap())?
        .files
        .vertices
        .get(&file_id)
        .map(|s| s.clone());

    if &current_user == &auther.unwrap()
    // || file.file_mode == FileMode::Public
    {
        let file_dir = FileDirectory::default();
        file_dir.files.push_children(file_dir.files.root.unwrap(), file.id, file);
        return Some(file_dir);
    };
    return None;
}
