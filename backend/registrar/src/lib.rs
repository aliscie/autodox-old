use std::{
    cell::RefCell,
    collections::HashMap
};

use ic_kit::{
    ic,
    Principal,
    macros::*,
    candid::{candid_method, export_service, CandidType}
};

mod instantiate_contract;
use instantiate_contract::*;

#[derive(CandidType)]
enum BackendError{
    InstantiateError(InstantiateError)
}

struct State{
    storage: HashMap<Principal, Vec<Principal>>,
}

impl Default for State{
    fn default() -> Self{
        Self{
            storage: HashMap::new()
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[init]
#[candid_method(init)]
fn init(){}

#[update]
#[candid_method(update)]
async fn register() -> Result<Principal, BackendError>{
    let caller = ic::caller();
    let address = create_storage(&caller).await
        .map_err(|e| BackendError::InstantiateError(e))?;
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        match state.storage.get_mut(&caller){
            None => {
                let storage = vec![address];
                state.storage.insert(caller, storage);
            },
            Some(list) => {
                list.push(address);
            }
        }
        Ok(address)
    })
}

#[query]
#[candid_method(query)]
fn get_storage_address(of: Principal) -> Vec<Principal>{
    STATE.with(|state|{
        let state = &state.borrow();
        let default_value: Vec<Principal> = Vec::new();
        let list = state.storage.get(&ic::caller()).unwrap_or(&default_value);
        list.clone()
    })
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("storage.did"), export_candid()).expect("Write failed.");
    }
}