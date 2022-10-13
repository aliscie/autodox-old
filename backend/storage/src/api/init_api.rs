use crate::structure::{InitData, State};

use std::cell::RefCell;


use ic_kit::{
    macros::*,
    candid::candid_method
};

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

#[init]
#[candid_method(init)]
pub fn init(init_data: InitData){
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        state.owner = init_data.owner;
    })
}