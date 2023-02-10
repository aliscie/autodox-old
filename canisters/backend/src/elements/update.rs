use crate::elements::types::ElementTrees;
use candid::candid_method;
use ic_cdk::update;
use ic_stable_memory::s;
use serde::Deserialize;
use shared::id::Id;
use shared::schema::{ElementTree, FileDirectory};

use crate::users::types::{User, UserFiles};

#[derive(Deserialize, Debug)]
struct ElementTreeCreate {
    file_id: Id,
    data: ElementTree,
}

#[update]
#[candid_method(update)]
pub fn create_element_tree(data: String) -> Result<(), String> {
    let ElementTreeCreate { data, file_id } =
        serde_json::from_str::<ElementTreeCreate>(&data).map_err(|_| "Serde errror".to_string())?;
    let user = User::current().ok_or("user not found")?;
    let mut user_files: UserFiles = s!(UserFiles);
    let file_directory = user_files.get_mut(&user).unwrap();
    let mut element_trees: ElementTrees = s!(ElementTrees);
    let mut file_node = file_directory
        .files
        .vertices
        .get_mut(&file_id)
        .ok_or("file_id not found")?;
    file_node.element_tree = Some(data.id);
    element_trees.insert(data.id, data);
    Ok(())
}
