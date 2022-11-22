use speedy::{Readable, Writable};
use candid::{CandidType, Deserialize};
use ic_stable_memory::utils::ic_types::SPrincipal;

use std::collections::HashMap;

use crate::response::{AddViewerResponse, RemoveViewerResponse};

use super::{UpdateFileData, DeleteFileData, ChildContentData, AddViewerData, RemoveViewerData, ChangeFileModeData};

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

pub fn get_username(address: SPrincipal, users: &Vec<User>) -> Option<String>{
    for user in users{
        if user.address == address{
            return Some(user.user_name.clone())
        }
    }
    None
}

#[derive(CandidType, Readable, Writable, Deserialize)]
pub enum FileMode{
    Public,
    Private,
    Restricted,
}

#[derive(Readable, Writable, CandidType)]
pub struct ChildContent{
    pub content: String,
}

impl ChildContent{
    pub fn new(content: String) -> Self{
        Self{
            content
        }
    }

    // pub fn get_content(&self) -> String{
    //     self.content.clone()
    // }

    pub fn update_content(&mut self, new_content: String){
        self.content = new_content;
    }
}

#[derive(Readable, Writable)]
pub struct ParentContent{
    pub name: String,
    pub parent_id: String,
    pub created_by: String,
    pub mode: FileMode,
    pub allowed_viewers: Vec<String>,
    pub content: String,
    pub child_contents: HashMap<String, ChildContent>
}

impl ParentContent{
    pub fn new(name: String, parent_id: String, created_by: String, mode: FileMode, content: String) -> Self{
        Self{
            name,
            parent_id,
            created_by,
            mode,
            allowed_viewers: Vec::new(),
            content,
            child_contents: HashMap::new()
        }
    }

    pub fn add_viewer(&mut self, username: String){
        self.allowed_viewers.push(username);
    }

    pub fn check_user(&self, name: String) -> bool{
        if self.allowed_viewers.contains(&name){
            return true
        }
        false
    }

    pub fn change_mode(&mut self, mode: FileMode){
        self.mode = mode;
    }

    pub fn remove_viewer(&mut self, username: String){
        self.allowed_viewers.retain(|user| *user != username);
    }

    pub fn update_content(&mut self, new_content: String){
        self.content = new_content;
    }

    pub fn add_child_content(&mut self, child_id: String, child_content: ChildContent){
        self.child_contents.insert(child_id, child_content);
    }

    // pub fn get_child_content(&mut self, child_id: String) -> Option<String>{
    //     match self.child_contents.get(&child_id){
    //         None => None,
    //         Some(child_content) => Some(child_content.get_content())
    //     }
    // }

    pub fn update_child_content(&mut self, child_id: String, new_content: String) -> Option<()>{
        match self.child_contents.get_mut(&child_id){
            None => None,
            Some(child_content) => {
                child_content.update_content(new_content);
                Some(())
            }
        }
    }
}

pub type UserFiles = Vec<ParentContent>;

pub enum UpdateResponse{
    Success,
    Unauthorized,
    FileDoesNotExist
}

pub fn update_content(caller: String, update_file_data: UpdateFileData, user_files: &mut UserFiles) -> UpdateResponse{
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == update_file_data.parent_id{
            if user_file.created_by != caller{
                return UpdateResponse::Unauthorized
            }
            match update_file_data.child_id{
                None => {
                    user_file.update_content(update_file_data.new_content);
                    return UpdateResponse::Success
                },
                Some(child_id) => {
                    match user_file.update_child_content(child_id, update_file_data.new_content){
                        None => return UpdateResponse::FileDoesNotExist,
                        Some(_) => return UpdateResponse::Success
                    }
                }
            }
        }
    }
    UpdateResponse::FileDoesNotExist
}

pub fn _add_child_content(caller: String, add_child_data: ChildContentData, user_files: &mut UserFiles) -> UpdateResponse{
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == add_child_data.parent_id{
            if user_file.created_by != caller{
                return UpdateResponse::Unauthorized
            }
            let child_content = ChildContent::new(add_child_data.content);
            user_file.add_child_content(add_child_data.child_id, child_content);
            return UpdateResponse::Success
        }
    }
    UpdateResponse::FileDoesNotExist
}

pub fn delete_file_or_content(caller: String, delete_file_data: DeleteFileData, user_files: &mut UserFiles) -> UpdateResponse{
    let mut delete_whole_file = false;
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == delete_file_data.parent_id{
            if user_file.created_by != caller{
                return UpdateResponse::Unauthorized
            }
            match delete_file_data.child_id{
                None => { delete_whole_file = true},
                Some(child_id) => {
                    user_file.child_contents.remove(&child_id);
                    return UpdateResponse::Success
                }
            }
        }
    }
    if !delete_whole_file{
        UpdateResponse::FileDoesNotExist
    }else{
        user_files.retain(|file| file.parent_id == delete_file_data.parent_id);
        UpdateResponse::Success
    }
}

pub enum FileError{
    NotAllowed,
    FileDoesNotExist
}

pub fn get_file_for_reading(caller: String, parent_id: String, user_files: UserFiles) -> Result<ParentContent, FileError>{
    for user_file in user_files{
        if user_file.parent_id == parent_id{
            match user_file.mode{
                FileMode::Public => return Ok(user_file),
                FileMode::Private => {
                    if user_file.created_by == caller{
                        return Ok(user_file)
                    }else{
                        return Err(FileError::NotAllowed)
                    }
                },
                FileMode::Restricted => {
                    if user_file.created_by == caller{
                        return Ok(user_file)
                    }
                    if !user_file.check_user(caller){
                        return Err(FileError::NotAllowed)
                    }
                    return Ok(user_file)
                }
            }
        }
    }
    Err(FileError::FileDoesNotExist)
}

pub fn is_this_user_valid(username: &String, users: Users) -> bool{
    let user_check = users.iter().any(|user| &user.user_name == username);
    if !user_check{
        return false
    }
    true
}

pub fn _add_viewer(caller: String, add_viewer_data: AddViewerData, user_files: &mut UserFiles) -> AddViewerResponse{
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == add_viewer_data.parent_id{
            if user_file.created_by != caller{
                return AddViewerResponse::Unauthorized;
            }
            if user_file.allowed_viewers.contains(&add_viewer_data.viewer){
                return AddViewerResponse::AlreadyExist
            }
            user_file.add_viewer(add_viewer_data.viewer);
            return AddViewerResponse::Success
        }
    }
    AddViewerResponse::FileDoesNotExist
}

pub fn _remove_viewer(caller: String, remove_viewer_data: RemoveViewerData, user_files: &mut UserFiles) -> RemoveViewerResponse{
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == remove_viewer_data.parent_id{
            if user_file.created_by != caller{
                return RemoveViewerResponse::Unauthorized
            }
            if !user_file.allowed_viewers.contains(&remove_viewer_data.viewer){
                return RemoveViewerResponse::ViewerNotFound
            }
            user_file.remove_viewer(remove_viewer_data.viewer);
            return RemoveViewerResponse::Success
        }
    }
    RemoveViewerResponse::FileDoesNotExist
}

pub fn _change_mode(caller: String, change_mode_data: ChangeFileModeData, user_files: &mut UserFiles) -> UpdateResponse{
    for user_file in user_files.iter_mut(){
        if user_file.parent_id == change_mode_data.parent_id{
            if user_file.created_by != caller{
                return UpdateResponse::Unauthorized
            }
            user_file.change_mode(change_mode_data.new_mode);
            return UpdateResponse::Success
        }
    }
    UpdateResponse::FileDoesNotExist
}