use crate::users::types::{User, Users};
use candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::id::Id;
use shared::schema::UserQuery;

#[query]
#[candid_method(query)]
pub fn get_profile() -> Option<UserQuery> {
    let mut users = s!(Users);
    let caller = User::caller();
    for user in users {
        if &user.address == &caller {
            return Some(UserQuery {
                address: user.address.to_string(),
                image: user.image,
                username: user.username,
                ..UserQuery::default()
            });
        }
    }
    None
}
