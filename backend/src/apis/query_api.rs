use std::collections::HashMap;

use ic_kit::{candid::candid_method, ic, macros::*};
use ic_stable_memory::{s, utils::ic_types::SPrincipal};

use crate::{backend_error::BackendError, schema::*};

// create a validator function that runner errr if lis is non
// fn validate_list(list: &HashMap<String, String>, key: String, child_id: String) -> Result<(), BackendError> {
//     match list.get_cloned(&key) {
//         None => Err(BackendError::FileDoesNotExist),
//         Some(list) => match list.get_cloned(child_id) {
//             None => Err(BackendError::FileDoesNotExist),
//             Some(data) => Ok(data),
//         },
//     }
// }

#[query]
#[candid_method(query)]
pub fn read_file(read_file_data: ReadFileData) -> Result<String, BackendError> {
    let s_caller = SPrincipal(ic::caller());
    let storage = s!(Storage);
    match storage.get_cloned(&s_caller) {
        None => Err(BackendError::FileDoesNotExist),
        Some(list) => {
            if let Some(parent_id) = read_file_data.parent_id {
                // TODO replace this with
                // validate_list(list,&parent_id, &read_file_data.child_id)
                match list.get_cloned(&parent_id) {
                    None => Err(BackendError::FileDoesNotExist),
                    Some(list) => match list.get_cloned(&read_file_data.child_id) {
                        None => Err(BackendError::FileDoesNotExist),
                        Some(data) => Ok(data),
                    },
                }
            } else {
                let default_key = String::new();
                // TODO replace this with
                // validate_list(list,&default_key, &read_file_data.child_id)
                match list.get_cloned(&default_key) {
                    None => Err(BackendError::FileDoesNotExist),
                    Some(list) => match list.get_cloned(&read_file_data.child_id) {
                        None => Err(BackendError::FileDoesNotExist),
                        Some(data) => Ok(data),
                    },
                }
            }
        }
    }
}

#[query]
#[candid_method(query)]
fn read_files() -> HashMap<String, HashMap<String, String>> {
    let s_caller = SPrincipal(ic::caller());
    let storage = s!(Storage);
    let mut result_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    match storage.get_cloned(&s_caller) {
        None => result_map,
        Some(map) => {
            let mut map = map.iter();
            while let Some((parent_id, child_map)) = map.next() {
                let mut _child_map: HashMap<String, String> = HashMap::new();
                let mut child_map = child_map.iter();
                while let Some((child_id, content)) = child_map.next() {
                    _child_map.insert(child_id, content);
                }
                result_map.insert(parent_id, _child_map);
            }
            result_map
        }
    }
}
