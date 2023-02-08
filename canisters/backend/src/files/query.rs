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
enum PermissionTypes{
    // Create,
    Read,
    Update,
    // Delete,
}

struct RestrictedPermission {
    any_one: bool,
    user: User,
    permission: Vec<PermissionTypes>
}

#[query]
#[candid_method(query)]
pub async fn get_directory(data: String) -> Option<FileDirectory> {
    let mut user_files: UserFiles = s!(UserFiles);
    let users: Users = s!(Users);

    let (auther_id, file_id) = serde_json::from_str::<(String, Id)>(&data).unwrap();
    let current_user = User::current()?;

    for auther in users {
        if auther.address.to_string() == auther_id {
            let directories = user_files.get(&auther)?; // TODO why this return None?
            let file = directories
                .files
                .vertices
                .get(&file_id)
                .map(|s| s.clone());

            if &current_user == &auther
                // || file.file_mode == FileMode::Public || file.permeted_users.contains(&current_user)
            {
            let mut file_dir = FileDirectory::default().await;
            file_dir.files.push_children(file_dir.files.root.unwrap(), file.clone().unwrap().id, file.unwrap());
            return Some(file_dir);
            };
        };
    };

    return None;
}
