use candid::candid_method;
use ic_cdk::export::Principal;
// use ic_cdk_macros::update;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use crate::custom_traits::*;

use ic_kit::{
    macros::update,
};

use crate::users::types::{User, Users};
use crate::utils::{Status, UpdateResponse};


#[update]
#[candid_method(update)]
pub fn register(user_name: String) -> UpdateResponse {
    let mut users = s!(Users);
    let caller: Option<User> = User::new();
    if caller.is_none() {
        return UpdateResponse { status: Status::UnAuthorized, message: "Please try to login".to_string() };
    }

    users.push(caller.unwrap());
    s! { Users = users}
    ;
    UpdateResponse { status: Status::Success, message: "ok".to_string() }
}

use serde::{Serialize};
use candid::{CandidType, Deserialize};

#[derive(Deserialize, Serialize, Debug, CandidType)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct UpdateProfile {
    pub user_name: Option<String>,
    pub image: Option<Vec<u8>>,
}

#[update]
#[candid_method(update)]
pub fn update_profile(data: UpdateProfile) -> UpdateResponse {
    let mut users = s!(Users);

    // for user in profile {
    //     if user.address = caller {
    //         user.update()
    //     }
    // }
    // users.push(new_user);

    s! { Users = users}
    ;
    UpdateResponse { status: Status::Success, message: "Your profile has been s updated.".to_string() }
}