use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types::{MyStringsSlice, MyStrings};

#[update]
fn add_my_string(entry: String) {
    let mut my_strings = s!(MyStrings);

    // this call now pushes new value directly to stable memory
    my_strings.push(&entry);

    // only saves SVec's pointer, instead of the whole collection
    s! { MyStrings = my_strings }
    ;
}


// #[update]
// #[candid_method(update)]
// pub fn create_file(create_file_data: shared::FileNode) -> CreateFileResponse{
//     let caller: SPrincipal = SPrincipal(ic_cdk::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return CreateFileResponse::UserNotRegisted,
//         Some(username) => username,
//     };
//     let mut user_files = s!(UserFiles);
//     let parent_content = ParentContent::new(create_file_data.name, create_file_data.parent_id, username, create_file_data.mode, create_file_data.content);
//     user_files.push(parent_content);
//     s!{ UserFiles = user_files};
//     CreateFileResponse::Success
// }