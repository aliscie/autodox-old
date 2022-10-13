use crate::{
    error::BackendError,
    structure::*,
};
use super::STATE;

use ic_kit::{
    ic,
    macros::*,
    Principal,
    candid::candid_method
};
use shared::Tree;

#[update]
#[candid_method(update)]
pub fn add_authority_for_making_changes(allowed_person: Principal) -> Result<(), BackendError>{
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        let caller = ic::caller();
        state.is_this_caller_owner(&caller)?;
        state.add_authority_for_making_changes(allowed_person)?;
        Ok(())
    })
}

#[update]
#[candid_method(update)]
pub fn remove_authority_from_making_changes(authority_to_remove: Principal) -> Result<(), BackendError>{
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        let caller = ic::caller();
        state.is_this_caller_owner(&caller)?;
        state.remove_authority_from_making_changes(authority_to_remove)?;
        Ok(())
    })
}

#[update]
#[candid_method(update)]
pub fn create_file(create_file_data: CreateFileData){
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        let mut new_tree: Tree<String, Data> = Tree::new();
        // new_tree.push_children()
        unimplemented!()
    })
}

#[update]
#[candid_method(update)]
pub fn change_directory(change_directory_data: ChangeDirectoryData) -> Result<(), BackendError>{
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        // let current_data = 
        unimplemented!()
    })
}

#[update]
#[candid_method(update)]
pub fn update_file(update_file_data: UpdateFileData) -> Result<(), BackendError>{
    unimplemented!()
}

#[update]
#[candid_method(update)]
pub fn delete_file(delete_file_data: DeleteFileData) -> Result<(), BackendError>{
    unimplemented!()
}