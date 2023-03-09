use std::collections::HashMap;
use crate::elements::types::ElementTrees;
use candid::candid_method;
use ic_cdk::update;
use ic_stable_memory::s;
use serde::Deserialize;
use shared::id::Id;
use shared::schema::{ElementTree, FileDirectory};

use crate::users::types::{User, UserFiles};

#[derive(Deserialize, Debug)]
pub struct ElementTreeCreate {
    file_id: Id,
    data: ElementTree,
}

#[update]
#[candid_method(update)]
pub fn create_element_tree(data: String) -> Result<String, String> {
    let res = serde_json::from_str::<ElementTreeCreate>(&data);
    if res.is_err() {
        return Ok(format!("Error: {:?}", res.err()));
    };
    let res = res.unwrap();
    let data = res.data;
    let file_id = res.file_id;

    let user = User::current().expect("Anonymous user.");
    let mut user_files: UserFiles = s!(UserFiles);
    let file_directory = user_files.get_mut(&user).unwrap();
    let mut element_trees: ElementTrees = s!(ElementTrees);
    let mut file_node = file_directory
        .files
        .vertices
        .get_mut(&file_id).expect("Anonymous user.");
    file_node.element_tree = Some(data.id);
    let mut new_element = HashMap::new();
    new_element.insert(data.id, data);
    element_trees.insert(user, new_element);
    s! { UserFiles = user_files }
    s! { ElementTrees = element_trees }
    Ok("ok".to_string())
}
