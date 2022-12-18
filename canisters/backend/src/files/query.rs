// use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileDirectory;
use crate::files::types::{MyStringsSlice, MyStrings};
// use crate::files::utils::{_get_directories};
use crate::utils::{BackendError, get_username};
use crate::users::types::{Users};

use ic_kit::{
    macros::*, candid::candid_method
};


#[query]
#[candid_method(query)]
pub fn get_directories(from: u64, to: u64) -> Option<Result<Vec<shared::schema::FileDirectory>, BackendError>> {
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return Some(Err(BackendError::UserNotRegisted)),
        Some(username) => username
    };
    // Ok(_get_directories(&username))
    None
}

