use crate::users::types::{User, Users};
use candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::{UserQuery, TestQuery};

#[query]
#[candid_method(query)]
pub fn get_profile() -> Option<UserQuery> {
    let mut users = s!(Users);
    let caller = User::caller();
    for user in users {
        if &user.address == &caller {
            return Some(UserQuery {
                image: user.image,
                username: user.username,
            });
        }
    }
    None
}

#[query]
#[candid_method(query)]
pub fn get_current_user() -> Option<UserQuery> {
    let user = User::current().unwrap();
    Some(UserQuery {
        image: user.image,
        username: user.username,
    })
}

#[query]
#[candid_method(query)]
pub fn get_test() -> Option<TestQuery> {
    Some(TestQuery {
        image: Some("image".to_string()),
        username: Some("username".to_string()),
    })
}
