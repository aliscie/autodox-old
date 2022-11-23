use ic_kit::{
    candid::candid_method,
    macros::*,
};
use ic_stable_memory::{
    s, utils::ic_types::SPrincipal
};

use crate::{structure::*, response::*};

#[query]
#[candid_method(query)]
pub fn username() -> UserNameResponse{
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    if let Some(name) = check_already_registered(&caller, users.clone()){
        return UserNameResponse::User { user_name: name }
    }
    UserNameResponse::UserNotRegisted
}

#[query]
#[candid_method(query)]
pub fn read_file(read_file_data: ReadFileData) -> ReadFileResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return ReadFileResponse::UserNotRegisted,
        Some(username) => username,
    };
    let user_files = s!(UserFiles);
    match get_file_for_reading(username, read_file_data.parent_id, user_files){
        Err(e) => {
            match e{
                FileError::FileDoesNotExist => return ReadFileResponse::FileDoesNotExist,
                FileError::NotAllowed => return ReadFileResponse::NotAllowed
            }
        },
        Ok(parent_content) => {
            ReadFileResponse::generate_file_response(parent_content.parent_id, parent_content.name, parent_content.content, parent_content.child_contents)
        }
    }
}

// #[query]
// #[candid_method(query)]
// pub fn read_content(read_content_data: ReadContentData) -> ReadContentResponse{
//     let caller: SPrincipal = SPrincipal(ic::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return ReadFileResponse::UserNotRegisted,
//         Some(username) => username,
//     };
//     let user_files = s!(UserFiles);
//     unimplemented!()
// }
