use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use crate::users::types::{User,Users};

pub fn get_user() -> String {
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    match get_username(caller, &users) {
        None => "is_longed_in".to_string(), //  CreateFileResponse::UserNotRegisted,
        Some(username) => "not_longed_in".to_string() // username,
    }
}

pub type UserName = String;

pub fn get_username(address: SPrincipal, users: &Vec<User>) -> Option<UserName> {
    for user in users {
        if user.address == address {
            return Some(user.user_name.clone());
        }
    }
    None
}
