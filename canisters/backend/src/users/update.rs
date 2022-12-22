use candid::candid_method;
use ic_cdk::export::Principal;
use ic_cdk_macros::update;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use crate::custom_traits::*;

use crate::users::types::{User, Users};
use crate::users::utils::is_registered;


#[update]
#[candid_method(update)]
pub fn register(user_name: String) -> String {
    println!("{:#?}", ic_cdk::caller());
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let _caller: Principal = ic_cdk::caller();
    let mut users = s!(Users);
    if Principal::is_anonymous() {
        "Please login with the IC identity.".to_string();
    }
    if let Some(registered_name) = is_registered(&caller, users.clone()) {
        return "already exits".to_string();//RegisterResponse::AlreadyRegistered { user_name: registered_name };
    }
    let new_user = User { user_name: user_name.clone(), address: caller.clone() };
    users.push(new_user);
    s! { Users = users}
    ;
    "ok".to_string()
}
