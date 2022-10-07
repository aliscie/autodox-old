use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use ic_kit::{
    macros::*,
    candid::{export_service, candid_method, CandidType},
};

#[derive(CandidType)]
enum BackendError{
    FailedToSerializeData,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Data{
    name: String,
    id: String,
    content: String,
    children: Vec<String>,
    parent: Option<String>,
}

struct State{
    list: Vec<Data>
}

impl Default for State{
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[init]
#[candid_method(init)]
fn init(){}

#[update(name = "create_file")]
#[candid_method(update, rename = "create_file")]
fn create_file(data: Vec<u8>) -> Result<(), BackendError>{
    let data: Data = serde_json::from_slice(&data).map_err(|_| BackendError::FailedToSerializeData)?;
    STATE.with(|state|{
        let state = &mut state.borrow_mut();
        state.list.push(data);
        Ok(())
    })
}

#[query(name = "read_file")]
#[candid_method(query, rename = "read_file")]
fn read_file() -> Vec<Data>{
    STATE.with(|state|{
        state.borrow().list.clone()
    })
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("backend.did"), export_candid()).expect("Write failed.");
    }
}