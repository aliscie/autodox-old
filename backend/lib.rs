mod files;
mod users;

use std::cell::RefCell;

use ic_kit::{
    macros::*,
    candid::{export_service, candid_method, CandidType},
};

#[derive(CandidType)]
enum BackendError {
    UnknownError,
}

#[derive(CandidType, Clone)]
struct Data {
    name: String,
    id: String,
    content: String,
    children: Vec<String>,
    parent: Option<String>,
}

impl Data {
    fn new(name: String, id: String, content: String, children: Vec<String>, parent: Option<String>) -> Self {
        Self {
            name,
            id,
            content,
            children,
            parent,
        }
    }
}

struct State {
    list: Vec<Data>
}

impl Default for State {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[init]
#[candid_method(init)]
fn init() {}

#[update(name = "create_file")]
#[candid_method(update, rename = "create_file")]
fn create_file(name: String, id: String, content: String, children: Vec<String>, parent: Option<String>) -> Result<(), BackendError> {
    STATE.with(|state| {
        let state = &mut state.borrow_mut();
        let data = Data::new(name, id, content, children, parent);
        state.list.push(data);
        Ok(())
    })
}

#[query(name = "read_file")]
#[candid_method(query, rename = "read_file")]
fn read_file() -> Vec<Data> {
    STATE.with(|state| {
        state.borrow().list.clone()
    })
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
