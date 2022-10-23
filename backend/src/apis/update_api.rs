use ic_kit::{
    macros::*,
    candid::candid_method, ic,
};

use crate::{structure::*, backend_error::BackendError};
use ic_stable_memory::{
    utils::ic_types::SPrincipal, s, collections::hash_map::SHashMap,
};

#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: CreateFileData){
    let mut storage = s!(Storage);
    let s_caller = SPrincipal(ic::caller());
    match storage.get_cloned(&s_caller){
        None =>{
            let mut string_map = SHashMap::new();
            string_map.insert(create_file_data.child_id, &create_file_data.content);
            if let Some(parent_id) = create_file_data.parent_id{
                let mut parent_map = SHashMap::new();
                parent_map.insert(parent_id, &string_map);
                storage.insert(s_caller, &parent_map);
            }
        },
        Some(mut list) => {
            if let Some(parent_id) = create_file_data.parent_id{
                match list.get_cloned(&parent_id){
                    None => {
                        let mut child_map = SHashMap::new();
                        child_map.insert(create_file_data.child_id, &create_file_data.content);
                        list.insert(parent_id, &child_map);
                        storage.insert(s_caller, &list);
                    },
                    Some(mut child_list) => {
                        child_list.insert(create_file_data.child_id, &create_file_data.content);
                        list.insert(parent_id, &child_list);
                        storage.insert(s_caller, &list);
                    }
                }
            }else{
                let default_key = String::new();
                match list.get_cloned(&default_key){
                    None =>{
                        let mut child_map = SHashMap::new();
                        child_map.insert(create_file_data.child_id, &create_file_data.content);
                        list.insert(default_key, &child_map);
                        storage.insert(s_caller, &list);
                    },
                    Some(mut child_list) => {
                        child_list.insert(create_file_data.child_id, &create_file_data.content);
                        list.insert(default_key, &child_list);
                        storage.insert(s_caller, &list);
                    }
                }
            }
        }
    }
    s!(Storage = storage);
}

#[update]
#[candid_method(update)]
pub fn update_file(update_file_data: UpdateFileData) -> Result<(), BackendError>{
    let s_caller = SPrincipal(ic::caller());
    let mut storage = s!(Storage);
    match storage.get_cloned(&s_caller){
        None => return Err(BackendError::FileDoesNotExist),
        Some(mut parent_map) => {
            if let Some(parent_id) = update_file_data.parent_id{
                match parent_map.get_cloned(&parent_id){
                    None => return Err(BackendError::FileDoesNotExist),
                    Some(mut child_map) => {
                        match child_map.get_cloned(&update_file_data.child_id){
                            None => return Err(BackendError::FileDoesNotExist),
                            Some(mut msg) => {
                                let new_msg = msg.replace(update_file_data.old_text.as_str(), update_file_data.new_text.as_str());
                                msg = new_msg;
                                child_map.insert(update_file_data.child_id, &msg);
                                parent_map.insert(parent_id, &child_map);
                                storage.insert(s_caller, &parent_map);
                            }
                        }
                    }
                }
            }else{
                let default_key = String::new();
                match parent_map.get_cloned(&default_key){
                    None => return Err(BackendError::FileDoesNotExist),
                    Some(mut child_map) => {
                        match child_map.get_cloned(&update_file_data.child_id){
                            None => return Err(BackendError::FileDoesNotExist),
                            Some(mut msg) => {
                                let new_msg = msg.replace(update_file_data.old_text.as_str(), update_file_data.new_text.as_str());
                                msg = new_msg;
                                child_map.insert(update_file_data.child_id, &msg);
                                parent_map.insert(default_key, &child_map);
                                storage.insert(s_caller, &parent_map);
                            }
                        }
                    }
                }
            }
        }
    }
    s!{ Storage = storage};
    Ok(())
}

#[update]
#[candid_method(update)]
pub fn delete_file(delete_file_data: DeleteFileData) -> Result<(), BackendError>{
    let s_caller = SPrincipal(ic::caller());
    let mut storage = s!(Storage);
    match storage.get_cloned(&s_caller){
        None => return Err(BackendError::FileDoesNotExist),
        Some(mut parent_map) => {
            if let Some(parent_id) = delete_file_data.parent_id{
                match parent_map.get_cloned(&parent_id){
                    None => return Err(BackendError::FileDoesNotExist),
                    Some(mut child_map) => {
                        match child_map.get_cloned(&delete_file_data.child_id){
                            None => return Err(BackendError::FileDoesNotExist),
                            Some(_) => {
                                child_map.remove(&delete_file_data.child_id);
                                parent_map.insert(parent_id, &child_map);
                                storage.insert(s_caller, &parent_map);
                            }
                        }
                    }
                }
            }else{
                let default_key = String::new();
                match parent_map.get_cloned(&default_key){
                    None => return Err(BackendError::FileDoesNotExist),
                    Some(mut child_map) =>{
                        match child_map.get_cloned(&delete_file_data.child_id){
                            None => return Err(BackendError::FileDoesNotExist),
                            Some(_) => {
                                child_map.remove(&delete_file_data.child_id);
                                parent_map.insert(default_key, &child_map);
                                storage.insert(s_caller, &parent_map);
                            }
                        }
                    }    
                }
            }
        }
    }
    s!{ Storage = storage};
    Ok(())
}