use ic_kit::{
    Principal,
    macros::*,
    candid::candid_method,
};

use crate::error::BackendError;
use super::STATE;

#[query]
#[candid_method(query)]
pub fn allowed_to_make_changes(address: Principal) -> Result<(), BackendError>{
    STATE.with(|state|{
        let state = &state.borrow();
        state.is_this_caller_allowed_to_update_content(&address)?;
        Ok(())
    })
}

#[query]
#[candid_method(query)]
pub fn read_content() -> Result<String, BackendError>{
    unimplemented!()
}