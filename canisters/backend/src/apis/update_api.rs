use ic_kit::{
    macros::*,
    candid::candid_method,
};

use ic_stable_memory::{
    s, utils::ic_types::SPrincipal,
};

use shared::{
    schema::*, id::Id
};

use crate::*;

#[update]
#[candid_method(update)]
pub fn create_directory(directory_id: Id, name: String) -> CreateDirectoryResponse{
    unimplemented!()
}

#[update]
#[candid_method(update)]
pub fn create_file(fnc: FileNodeCreate) -> Result<(), BackendError>{
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return Err(BackendError::UserNotRegisted),
        Some(username) => username
    };
    unimplemented!()
}

#[update]
#[candid_method(update)]
pub fn register(user_name: String) -> RegisterResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let mut users = s!(Users);
    if let Some(registered_name) = check_already_registered(&caller, users.clone()){
        return RegisterResponse::AlreadyRegistered { user_name: registered_name }
    }
    if username_check(&user_name, &users){
        return RegisterResponse::UserNameAlreadyInUse
    }
    let new_user = User::new(&user_name, &caller);
    users.push(new_user);
    s!{ Users = users};
    RegisterResponse::Success { user_name }
}