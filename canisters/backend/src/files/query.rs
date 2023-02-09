use crate::elements::types::ElementTrees;
use crate::users::types::Users;
use crate::users::types::{User, UserFiles};
use ic_kit::candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use shared::{id::Id, schema::*, Tree};
use std::collections::HashMap;

#[query]
#[candid_method(query)]
pub fn get_directories() -> Option<FileDirectory> {
    let user = User::current()?;
    let mut user_files: UserFiles = s!(UserFiles);
    user_files.get(&user).map(|s| s.clone())
}
enum PermissionTypes {
    // Create,
    Read,
    Update,
    // Delete,
}

struct RestrictedPermission {
    any_one: bool,
    user: User,
    permission: Vec<PermissionTypes>,
}

#[query]
#[candid_method(query)]
pub fn get_directory(data: String) -> Option<FileDirectory> {
    let mut user_files: UserFiles = s!(UserFiles);
    let users: Users = s!(Users);

    let (auther_id, file_id) = serde_json::from_str::<(String, Id)>(&data).unwrap();
    let current_user = User::current()?;

    for auther in users {
        if auther.address.to_string() == auther_id {
            let directories = user_files.get(&auther)?.clone();

            if &current_user == &auther
            // || file.file_mode == FileMode::Public || file.permeted_users.contains(&current_user)
            {
                let directories = user_files.get(&auther)?.clone();
                return Some(directories);
            };
        };
    }

    return None;
}

#[query]
#[candid_method(query)]
pub fn get_file_node(file_id: Id) -> Option<(FileNode, ElementTree)> {
    let user = User::current()?;
    let mut user_files: UserFiles = s! {UserFiles};
    let element_trees: ElementTrees = s!(ElementTrees);
    let files: UserFiles = s!(UserFiles);
    let file_node: FileNode = user_files
        .get(&user)
        .and_then(|directory| directory.files.vertices.get(&file_id))?
        .clone();
    let tree = file_node
        .element_tree
        .and_then(|element_tree_id| element_trees.get(&element_tree_id))?
        .clone();
    Some((file_node, tree))
}
