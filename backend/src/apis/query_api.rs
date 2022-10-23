use ic_kit::{
    candid::candid_method,
    macros::*, ic,
};
use ic_stable_memory::{
    s, utils::ic_types::SPrincipal
};
use crate::{structure::*, backend_error::BackendError};

#[query]
#[candid_method(query)]
pub fn read_file(read_file_data: ReadFileData) -> Result<String, BackendError>{
    let s_caller = SPrincipal(ic::caller());
    let storage = s!(Storage);
    match storage.get_cloned(&s_caller){
        None => Err(BackendError::FileDoesNotExist),
        Some(list) => {
            if let Some(parent_id) = read_file_data.parent_id{
                match list.get_cloned(&parent_id){
                    None => Err(BackendError::FileDoesNotExist),
                    Some(list) => {
                        match list.get_cloned(&read_file_data.child_id){
                            None => Err(BackendError::FileDoesNotExist),
                            Some(data) => Ok(data)
                        }
                    }
                }
            }else{
                let default_key = String::new();
                match list.get_cloned(&default_key){
                    None => Err(BackendError::FileDoesNotExist),
                    Some(list) => {
                        match list.get_cloned(&read_file_data.child_id){
                            None => Err(BackendError::FileDoesNotExist),
                            Some(data) => Ok(data)
                        }
                    }
                }
            }
        }
    }
}