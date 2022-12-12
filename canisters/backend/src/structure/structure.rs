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

pub type Files = HashMap<Id, Tree<Id, FileNode>>;

pub type UserFiles = HashMap<String, Files>;

pub enum CreateResult{
    Ok,
    AlreadyExist,
}

pub fn create_directory(user_files: &mut UserFiles, username: &String, directory_id: Id) -> CreateResult{
    unimplemented!()
}

pub fn create_file(user_files: &mut UserFiles, username: &String,) -> CreateResult{
    unimplemented!()
}

type ElementTreeStorage = HashMap<UserName, HashMap<Id, ElementTree>>;

pub fn get_element_tree(element_tree_storage: &ElementTree, username: UserName, id: Id) -> Option<ElementTree>{
    unimplemented!()
}