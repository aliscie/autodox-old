use ic_stable_memory::collections::vec::SVec;
use std::collections::HashMap;
pub type MyStrings = SVec<String>;
pub type MyStringsSlice = Vec<String>;
use candid::CandidType;
use serde::{Deserialize, Serialize};

use shared::{
    id::Id,
    schema::{ElementTree, FileDirectory, FileMode, FileNode},
    Tree,
};
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
