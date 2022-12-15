use ic_kit::{
    candid::candid_method,
    macros::*,
};
use ic_stable_memory::{
    s, utils::ic_types::SPrincipal
};

use crate::{structure::*, response::*};


#[ic_cdk_macros::query]
#[candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


// use ic_kit::{
//     macros::*,
//     candid::candid_method,
// };
// use ic_stable_memory::{
//     s, utils::ic_types::SPrincipal
// };
//
//
// // use crate::{structure::*, response::*};
// // #[update]
// // #[candid_method(update)]
// // pub fn change_file_mode(change_file_mode_data: ChangeFileModeData) -> ChangeFileModeResponse{
// //     let caller: SPrincipal = SPrincipal(ic_cdk::caller());
// //     let users = s!(Users);
// //     let username = match get_username(caller, &users){
// //         None => return ChangeFileModeResponse::UserNotRegisted,
// //         Some(username) => username
// //     };
// //     let mut user_files = s!(UserFiles);
// //     match _change_mode(username, change_file_mode_data, &mut user_files){
// //         UpdateResponse::Success => {
// //             s!{ UserFiles = user_files};
// //             ChangeFileModeResponse::Success
// //         },
// //         UpdateResponse::Unauthorized => ChangeFileModeResponse::Unauthorized,
// //         UpdateResponse::FileDoesNotExist => ChangeFileModeResponse::FileDoesNotExist
// //     }
// // }
//
// // #[query]
// // #[candid_method(query)]
// // pub fn read_file(read_content_data: ReadContentData) -> ReadContentResponse{
// //     let caller: SPrincipal = SPrincipal(ic::caller());
// //     let users = s!(Users);
// //     let username = match get_username(caller, &users){
// //         None => return ReadFileResponse::UserNotRegisted,
// //         Some(username) => username,
// //     };
// //     let user_files = s!(UserFiles);
// //     unimplemented!()
// // }
//
// #[query]
// #[candid_method(query)]
// pub fn read_file(s: String) -> String {
//     return "Hello".to_string();
// }
//
// // #[cfg(test)]
// // mod tests{
// //     use super::*;
// //
// //     #[test]
// //     pub fn save_candid() {
// //         use std::env;
// //         use std::fs::write;
// //         use std::path::PathBuf;
// //
// //         let dir = PathBuf::from(env::current_dir().unwrap());
// //         write(dir.join("backend.did"), export_candid()).expect("Write failed.");
// //     }
// // }
