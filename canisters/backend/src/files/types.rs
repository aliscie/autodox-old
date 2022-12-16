use ic_stable_memory::collections::vec::SVec;
use std::collections::HashMap;
pub type MyStrings = SVec<String>;
pub type MyStringsSlice = Vec<String>;


use shared::{schema::{FileMode, FileDirectory, FileNode, ElementTree}, id::Id, Tree};
pub type Files = HashMap<Id, Tree<Id, FileNode>>;