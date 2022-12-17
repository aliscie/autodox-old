use candid::candid_method;
use ic_kit::{
    macros::*, candid::CandidType
};

use ic_stable_memory::{
    utils::ic_types::SPrincipal,
    s,
};

use shared::schema::*;

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
    get_element_tree(&element_tree_storage, username, id)
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
    Ok(get_directories(username))
}