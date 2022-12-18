use speedy::{Readable, Writable};
use ic_stable_memory::{
    utils::ic_types::SPrincipal, s
};
use shared::{schema::*, id::Id};

use std::collections::HashMap;

#[derive(Clone, Readable, Writable)]
pub struct User{
    pub user_name: String,
    pub address: SPrincipal,
}

impl User {
    pub fn new(name: &String, address: &SPrincipal) -> Self{
        Self{
            user_name: name.clone(),
            address: address.clone()
        }
    }

    pub fn change_username(&mut self, name: &String){
        self.user_name = name.clone();
    }
}

pub type Users = Vec<User>;

pub fn check_already_registered(address: &SPrincipal, mut list: Vec<User>) -> Option<String>{
    list.retain(|user| &user.address == address);
    if list.len() == 0{
        return None
    }
    Some(list.get(0).unwrap().user_name.clone())
}

pub fn username_check(name: &String, list: &Vec<User>) -> bool{
    list.iter().any(|user| &user.user_name == name)
}

pub type UserName = String;

pub fn get_username(address: SPrincipal, users: &Vec<User>) -> Option<UserName>{
    for user in users{
        if user.address == address{
            return Some(user.user_name.clone())
        }
    }
    None
}

pub type UserFiles = HashMap<String, Vec<FileDirectory>>;

pub enum CreateResult{
    Ok,
    AlreadyExist,
    DirectoryDoesNotExist
}

pub fn _create_directory(user_files: &mut UserFiles, username: &String, directory_id: Id, name: String) -> CreateResult{
    let fd = FileDirectory::new(directory_id, name);
    match user_files.get_mut(username){
        Some(files) => {
            if files.iter().any(|file| file.id == directory_id){
                return CreateResult::AlreadyExist
            }
            files.push(fd);
            return CreateResult::Ok
        },
        None => ()
    }
    let l_fd = vec![fd];
    user_files.insert(username.clone(), l_fd);
    return CreateResult::Ok
}

pub fn _create_file(user_files: &mut UserFiles, username: &String, directory_id: Id, id: Id, name: String, children: Option<Vec<Id>>) -> CreateResult{
    let filenode = FileNode{
        id,
        name,
        element_tree: None,
    };
    let mut default = Vec::new();
    let directories = user_files.get_mut(username).unwrap_or(&mut default);
    for directory in directories.iter_mut(){
        if directory.id == directory_id{
            directory.files.vertices.insert(filenode.id.clone(), filenode.clone());
            directory.files.adjacency.insert(filenode.id.clone(), children.unwrap_or(Vec::new()));
            return CreateResult::Ok
        }
    }
    CreateResult::DirectoryDoesNotExist
}

pub fn _get_directories(username: &String) -> Vec<FileDirectory>{
    let files = s!(UserFiles);
    match files.get(username){
        None => Vec::new(),
        Some(list) => list.clone(),
    }
}

pub type ElementTreeStorage = HashMap<UserName, HashMap<Id, ElementTree>>;

// pub fn _create_element_tree(element_tree_storage: &ElementTree, eec: EditorElementCreate) -> CreateResult{
//     let editor_element = EditorElement::from(eec);
//     unimplemented!()
// }

pub fn _get_element_tree(element_tree_storage: &ElementTreeStorage, username: UserName, id: Id) -> Option<ElementTree>{
    match element_tree_storage.get(&username){
        None => None,
        Some(element_trees) => {
            match element_trees.get(&id){
                None => None,
                Some(element_tree) => Some(element_tree.clone())
            }
        }
    }
}