use ic_kit::{
    macros::*,
    candid::candid_method,
};
use ic_stable_memory::{
    s, utils::ic_types::SPrincipal
};

use crate::{structure::*, response::*};

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

#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: CreateFileData) -> CreateFileResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return CreateFileResponse::UserNotRegisted,
        Some(username) => username,
    };
    let mut user_files = s!(UserFiles);
    let parent_content = ParentContent::new(create_file_data.name, create_file_data.parent_id, username, create_file_data.mode, create_file_data.content);
    user_files.push(parent_content);
    s!{ UserFiles = user_files};
    CreateFileResponse::Success
}

#[update]
#[candid_method(update)]
pub fn add_viewer(add_viewer_data: AddViewerData) -> AddViewerResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return AddViewerResponse::UserNotRegisted,
        Some(username) => username
    };
    if !is_this_user_valid(&add_viewer_data.viewer, users){
        return AddViewerResponse::InvalidUser
    }
    let mut user_files = s!(UserFiles);
    match _add_viewer(username, add_viewer_data, &mut user_files){
        AddViewerResponse::Success => {
            s!{ UserFiles = user_files};
            AddViewerResponse::Success
        },
        AddViewerResponse::Unauthorized => AddViewerResponse::Unauthorized,
        AddViewerResponse::AlreadyExist => AddViewerResponse::AlreadyExist,
        _ => AddViewerResponse::FileDoesNotExist
    }
}

#[update]
#[candid_method(update)]
pub fn remove_viewer(remove_viewer_data: RemoveViewerData) -> RemoveViewerResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return RemoveViewerResponse::UserNotRegisted,
        Some(username) => username
    };
    if !is_this_user_valid(&remove_viewer_data.viewer, users){
        return RemoveViewerResponse::InvalidUser
    }
    let mut user_files = s!(UserFiles);
    match _remove_viewer(username, remove_viewer_data, &mut user_files){
        RemoveViewerResponse::Success => {
            s!{ UserFiles = user_files};
            RemoveViewerResponse::Success
        },
        RemoveViewerResponse::ViewerNotFound => RemoveViewerResponse::ViewerNotFound,
        RemoveViewerResponse::Unauthorized => RemoveViewerResponse::Unauthorized,
        _ => RemoveViewerResponse::FileDoesNotExist
    }
}

#[update]
#[candid_method(update)]
pub fn add_child_content(child_content_data: ChildContentData) -> AddChildContentResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return AddChildContentResponse::UserNotRegisted,
        Some(username) => username,
    };
    let mut user_files = s!(UserFiles);
    match _add_child_content(username, child_content_data, &mut user_files){
        UpdateResponse::Success => {
            s!{ UserFiles = user_files};
            AddChildContentResponse::Success
        },
        UpdateResponse::FileDoesNotExist => AddChildContentResponse::FileDoesNotExist,
        UpdateResponse::Unauthorized => AddChildContentResponse::Unauthorized
    }
}

#[update]
#[candid_method(update)]
pub fn update_file(update_file_data: UpdateFileData) -> UpdateFileResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return UpdateFileResponse::UserNotRegisted,
        Some(username) => username,
    };
    let mut user_files = s!(UserFiles);
    match update_content(username, update_file_data, &mut user_files){
        UpdateResponse::Success => {
            s!{ UserFiles = user_files};
            UpdateFileResponse::Success
        },
        UpdateResponse::FileDoesNotExist => UpdateFileResponse::FileDoesNotExist,
        UpdateResponse::Unauthorized => UpdateFileResponse::Unauthorized
    }
}

#[update]
#[candid_method]
pub fn delete_file(delete_file_data: DeleteFileData) -> DeleteFileResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return DeleteFileResponse::UserNotRegisted,
        Some(username) => username
    };
    let mut user_files = s!(UserFiles);
    match delete_file_or_content(username, delete_file_data, &mut user_files){
        UpdateResponse::Success => {
            s!{ UserFiles = user_files};
            DeleteFileResponse::Success
        },
        UpdateResponse::FileDoesNotExist => DeleteFileResponse::FileDoesNotExist,
        UpdateResponse::Unauthorized => DeleteFileResponse::Unauthorized
    }
}

#[update]
#[candid_method(update)]
pub fn change_file_mode(change_file_mode_data: ChangeFileModeData) -> ChangeFileModeResponse{
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users){
        None => return ChangeFileModeResponse::UserNotRegisted,
        Some(username) => username
    };
    let mut user_files = s!(UserFiles);
    match _change_mode(username, change_file_mode_data, &mut user_files){
        UpdateResponse::Success => {
            s!{ UserFiles = user_files};
            ChangeFileModeResponse::Success
        },
        UpdateResponse::Unauthorized => ChangeFileModeResponse::Unauthorized,
        UpdateResponse::FileDoesNotExist => ChangeFileModeResponse::FileDoesNotExist
    }
}
