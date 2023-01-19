use candid::candid_method;
use ic_cdk::export::Principal;
// use ic_cdk_macros::update;
use crate::custom_traits::*;
use crate::users::types::{User, Users};
use crate::utils::{Status, UpdateResponse};
use ic_kit::macros::update;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;

#[update]
#[candid_method(update)]
pub fn register(username: String) -> String {
    let mut users = s!(Users);
    let caller: Option<User> = User::new();

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

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, CandidType)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct UpdateProfile {
    pub username: String,
    pub image: Vec<u8>,
}

// ic_stable struct
// https://docs.rs/ic-stable-structures/latest/ic_stable_structures/
// BTreeMap

#[update]
#[candid_method(update)]
pub fn update_profile(data: String) -> String {
// TODO pub fn update_profile(data: User) -> String {

    let profile_data = serde_json::from_str::<UpdateProfile>(&data).unwrap();
    let mut users = s!(Users);
    let caller = User::caller();
    for mut user in &mut users {
        if &user.address == &caller {
            user.image = Some(profile_data.image.clone());
            user.username = Some(profile_data.username.clone());
            s! { Users = users}
            ;
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
