use ic_kit::{
    macros::*,
    candid::candid_method,
};
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types;
use candid::CandidType;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileNode;
use crate::utils::{Status, UpdateResponse};


#[update]
#[candid_method(update)]
fn create_file(create_file_data: types::CreateFileData) -> Status {
    use crate::users::types::UserFiles;
    let mut user_files = s!(UserFiles);
    // user_files.push(HashMap::new(username, new_file_tree));
    s! { UserFiles = user_files}
    ;
    Status::Success
}
