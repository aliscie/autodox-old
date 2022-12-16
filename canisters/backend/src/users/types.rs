use std::collections::HashMap;
use ic_stable_memory::collections::vec::SVec;
use crate::files::types::Files;
use speedy::{Readable, Writable}; // TODO what is this does?
use ic_stable_memory::utils::ic_types::SPrincipal;

#[derive(Clone, Readable, Writable)]
pub struct User {
    pub user_name: String,
    pub address: SPrincipal,
}

pub type Users = Vec<User>;


pub type UserFiles = HashMap<String, Files>;