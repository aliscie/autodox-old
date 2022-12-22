use crate::files::types::Files;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileDirectory;
use speedy::{Readable, Writable}; // TODO what is this does?
use std::collections::HashMap;

#[derive(Clone, Readable, Writable)]
pub struct User {
    pub user_name: String,
    pub address: SPrincipal,
}

pub type Users = Vec<User>;

pub type UserFiles = HashMap<String, FileDirectory>;
