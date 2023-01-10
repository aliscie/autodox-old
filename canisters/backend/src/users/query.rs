use crate::users::types::{User, Users};
use candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::QueryUser;

#[query]
#[candid_method(query)]
pub fn get_profile() -> Option<QueryUser> {
    let mut users = s!(Users);
    let caller = User::caller();
    for user in users {
        if &user.address == &caller {
            return Some(QueryUser {
                image: user.image,
                username: user.username,
            });
        }
    }
    None
}
