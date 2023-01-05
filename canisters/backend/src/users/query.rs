use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use candid::candid_method;
use ic_kit::macros::query;
use crate::users::types::{User, Users};


#[query]
#[candid_method(query)]
pub fn get_profile() -> Option<Vec<u8>> { /// TODO return User cauz and error query not implement for User
    let mut users = s!(Users);
    let caller = User::caller();
    for user in users {
        if &user.address == &caller {
            return user.image;
        }
    };
    None
}