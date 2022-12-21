use ic_stable_memory::collections::vec::SVec;
use std::collections::HashMap;
pub type MyStrings = SVec<String>;
pub type MyStringsSlice = Vec<String>;
use serde::{Deserialize, Serialize};
use candid::CandidType;

use shared::{schema::{FileMode, FileDirectory, FileNode, ElementTree}, id::Id, Tree};
pub type Files = HashMap<Id, Tree<Id, FileNode>>;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "backend", derive(Readable, Writable))]

#[derive(CandidType, Deserialize)]
pub struct CreateFileData {
    pub parent_id: String,
    pub name: String,
    pub mode: FileMode,
    pub content: String,
}



pub enum CreateResult{
    Ok,
    AlreadyExist,
    DirectoryDoesNotExist
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}