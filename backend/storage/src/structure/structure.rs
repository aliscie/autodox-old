use ic_kit::{
    Principal,
    candid::{
    CandidType, Deserialize,
    }
};
use std::collections::HashMap;
use serde::Serialize;

// importing tree
use shared::Tree;
// use uuid::Uuid;
use crate::error::BackendError;

#[derive(CandidType, Deserialize)]
pub struct InitData{
    pub owner: Principal,
}

/*
 {
      "id":"3434-dfldkjfor9wir9023",
      "tag":"p",
      "children":[
         {
            "text":"this is a paragraph"
         },
         {
       "id":"334-d335or9wir9023",
            "tag":"span",
            "children":[
               {
                  "text":"this is a bold text",
                  "attributes":{
                     "style":{
                        "fontwait":"bold"
                     }
                  }
               }
            ]
         }
      ]
   }
*/

#[derive(CandidType, Deserialize)]
pub struct ContentUpdateData{

}

#[derive(CandidType, Eq, PartialEq, PartialOrd, Ord, Debug, Serialize, Clone)]
pub struct Data{

}

pub struct State{
    pub owner: Principal,
    pub allowed_to_update: Vec<Principal>,
    pub content: HashMap<String, Tree<String, Data>>,
}

impl Default for State{
    fn default() -> Self {
        Self{
            owner: Principal::anonymous(),
            allowed_to_update: Vec::new(),
            content: HashMap::new()
        }
    }
}

impl State{
    pub fn is_this_caller_owner(&self, caller: &Principal) -> Result<(), BackendError>{
        if *caller != self.owner{
            return Err(BackendError::Unauthorized)
        }
        Ok(())
    }

    pub fn is_this_caller_allowed_to_update_content(&self, caller: &Principal) -> Result<(), BackendError>{
        if *caller == self.owner || self.allowed_to_update.contains(caller){
            return Ok(())
        }
        Err(BackendError::Unauthorized)
    }

    pub fn add_authority_for_making_changes(&mut self, new_authority: Principal) -> Result<(), BackendError>{
        if self.allowed_to_update.contains(&new_authority){
            return Err(BackendError::AuthorityAlreadyExist)
        }
        self.allowed_to_update.push(new_authority);
        Ok(())
    }

    pub fn remove_authority_from_making_changes(&mut self, authority_to_remove: Principal) -> Result<(), BackendError>{
        if !self.allowed_to_update.contains(&authority_to_remove){
            return Err(BackendError::AuthorityDoesNotExist)
        }
        self.allowed_to_update.retain(|authority| *authority != authority_to_remove);
        Ok(())
    }
}

#[derive(CandidType, Deserialize)]
pub struct CreateFileData{
    pub tree_id: String,
    pub parent_id: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
    pub content_update_data: ContentUpdateData,
}

#[derive(CandidType, Deserialize)]
pub struct DeleteFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct ChangeDirectoryData{
    pub previous_parent_id: Option<String>,
    pub previous_child_id: String,
    pub new_parent_id: Option<String>,
    pub new_child_id: String,
}