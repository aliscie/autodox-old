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
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return CreateDirectoryResponse::UserNotRegisted,
        Some(username) => username
    };
    let mut user_files = s!(UserFiles);
    match _create_directory(&mut user_files, &username, directory_id, name){
        CreateResult::AlreadyExist => return CreateDirectoryResponse::AlreadyExist,
        _ => ()
    }
    s!{ UserFiles = user_files};
    CreateDirectoryResponse::Success
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
    let mut user_files = s!(UserFiles);
    match _create_file(&mut user_files, &username, fnc.directory_id, fnc.id, fnc.name, fnc.children){
        CreateResult::Ok => (),
        _ => return Err(BackendError::DirectoryDoesNotExist)
    }
    s!{ UserFiles = user_files};
    Ok(())
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