use speedy::{Readable, Writable};
use ic_stable_memory::{
    utils::ic_types::SPrincipal, s
};
use shared::{schema::*, id::Id};

use std::collections::HashMap;
use crate::users::types::UserFiles;

// pub fn _get_directories(username: &String) -> Vec<FileDirectory>{
//     let files = s!(UserFiles);
//     match files.get(username){
//         None => Vec::new(),
//         Some(list) => list.clone(),
//     }
// }