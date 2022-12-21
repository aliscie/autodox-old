use ic_kit::{
    candid::candid_method,
};
use ic_cdk::update;
use ic_stable_memory::s;
use ic_stable_memory::utils::ic_types::SPrincipal;
use crate::files::types::RegisterResponse;
use crate::files::utils::{check_already_registered, username_check};
use crate::users;
use crate::users::types::{User, Users};
use crate::utils::Status;

// use crate::update;
// use ic_stable_memory::{
//     s, utils::ic_types::SPrincipal,
// };
//
// use shared::{
//     schema::*, id::Id,
// };
//
// // use crate::*;
// use crate::files::types::RegisterResponse;

// #[update]
// #[candid_method(update)]
// pub fn create_directory(directory_id: Id, name: String) -> CreateDirectoryResponse{
//     let caller = SPrincipal(ic_cdk::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return CreateDirectoryResponse::UserNotRegisted,
//         Some(username) => username
//     };
//     let mut user_files = s!(UserFiles);
//     match _create_directory(&mut user_files, &username, directory_id, name){
//         CreateResult::AlreadyExist => return CreateDirectoryResponse::AlreadyExist,
//         _ => ()
//     }
//     s!{ UserFiles = user_files};
//     CreateDirectoryResponse::Success
// }

// #[update]
// #[candid_method(update)]
// pub fn create_file(fnc: FileNodeCreate) -> Result<(), BackendError>{
//     let caller = SPrincipal(ic_cdk::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return Err(BackendError::UserNotRegisted),
//         Some(username) => username
//     };
//     let mut user_files = s!(UserFiles);
//     match _create_file(&mut user_files, &username, fnc.directory_id, fnc.id, fnc.name, fnc.children){
//         CreateResult::Ok => (),
//         _ => return Err(BackendError::DirectoryDoesNotExist)
//     }
//     s!{ UserFiles = user_files};
//     Ok(())
// }


#[update]
#[candid_method(update)]
pub fn register(user_name: String) -> String {
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let mut users = s!(Users);
    if let Some(registered_name) = check_already_registered(&caller, users.clone()) {
        return "already exits".to_string();//RegisterResponse::AlreadyRegistered { user_name: registered_name };
    }
    let new_user = User { user_name: user_name.clone(), address: caller.clone() };
    users.push(new_user);
    s! { Users = users}
    ;
    "ok".to_string()
}