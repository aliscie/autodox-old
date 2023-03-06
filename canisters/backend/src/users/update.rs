use candid::candid_method;
use ic_cdk::export::Principal;
use shared::schema::UserQuery;
// use ic_cdk_macros::update;
use crate::custom_traits::*;
use crate::users::types::{User, Users};
use crate::utils::{Status, UpdateResponse};
use crate::ElementTree;
use crate::FileDirectory;
use crate::FileNode;
use candid::export_service;
use ic_kit::macros::update;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::id::Id;
use std::collections::HashMap;

#[update]
#[candid_method(update)]
pub fn register(username: String) -> String {
    let mut users = s!(Users);
    let mut caller: Option<User> = User::new();

    if User::is_anonymous() {
        // return UpdateResponse {
        //     status: Status::UnAuthorized,
        //     message: "Anonymous user.".to_string(),
        // };
        return "Anonymous user.".to_string();
    };

    if User::is_registered() {
        // return UpdateResponse {
        //     status: Status::InvalidInput,
        //     message: "User already registered".to_string(),
        // };
        return "User already registered".to_string();
    };

    if caller.is_none() {
        // return UpdateResponse {
        //     status: Status::UnAuthorized,
        //     message: "Please try to login".to_string(),
        // };
        return "Please try to login".to_string();
    }
    users.push(caller.unwrap());
    s! { Users = users};
    // UpdateResponse {
    //     status: Status::Success,
    //     message: "ok".to_string(),
    // }
    return "ok".to_string();
}

#[update]
#[candid_method(update)]
pub fn update_profile(data: String) -> String {
    // TODO pub fn update_profile(data: User) -> String {

    let profile_data = serde_json::from_str::<UserQuery>(&data).unwrap();
    let mut users = s!(Users);
    let caller = User::caller();
    for mut user in &mut users {
        if &user.address == &caller {
            user.image = profile_data.image;
            user.username = profile_data.username;
            s! { Users = users};
            // return UpdateResponse {
            //     status: Status::Success,
            //     message: "Your profile has been s updated.".to_string(),
            // };
            return "Your profile has been s updated.".to_string();
        }
    }
    // UpdateResponse {
    //     status: Status::UnAuthorized,
    //     message: "User not registered.".to_string(),
    // }
    "User not registered.".to_string()
}

export_service!();

#[test]
fn export_did() {
    let expected = __export_service();
    std::fs::write("backend.did", expected).unwrap();
}
