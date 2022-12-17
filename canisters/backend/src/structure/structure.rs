use speedy::{Readable, Writable};
use candid::CandidType;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::{schema::{FileMode, FileDirectory, FileNode, ElementTree}, id::Id, Tree};

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
}

pub fn create_directory(user_files: &mut UserFiles, username: &String, directory_id: Id, name: String) -> CreateResult{
    let fd = FileDirectory::new(id, name);
    match user_files.get_mut(username){
        Some(files) => {
            files.push(fd);
            return CreateResult::Ok
        },
        None => ()
    }
    let l_fd = vec![fd];
    user_files.insert(*username, l_fd);
    return CreateResult::Ok
}

pub fn get_directories(username: &String) -> Vec<FileDirectory>{
    let files = s!(UserFiles);
    match files.get(username){
        None => Vec::new(),
        Some(list) => list,
    }
}

type ElementTreeStorage = HashMap<UserName, HashMap<Id, ElementTree>>;

pub fn get_element_tree(element_tree_storage: &ElementTree, username: UserName, id: Id) -> Option<ElementTree>{
    match element_tree_storage.get(&username){
        None => None,
        Some(element_trees) => {
            match element_trees.get(&id){
                None => None,
                Some(element_tree) => Some(element_tree)
            }
        }
    }
}