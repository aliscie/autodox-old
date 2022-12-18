use ic_kit::{
    macros::*, candid::candid_method
};

use ic_stable_memory::{
    utils::ic_types::SPrincipal,
    s,
};

use shared::{schema::*, id::Id};

use crate::*;

#[query]
#[candid_method(query)]
pub fn read_element(id: Id) -> Option<ElementTree>{
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return None,
        Some(username) => username
    };
    let element_storage_tree = s!(ElementTreeStorage);
    _get_element_tree(&element_storage_tree, username, id)
}

#[query]
#[candid_method(query)]
pub fn read_files() -> Result<Vec<FileDirectory>, BackendError>{
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return Err(BackendError::UserNotRegisted),
        Some(username) => username
    };
    Ok(_get_directories(&username))
}