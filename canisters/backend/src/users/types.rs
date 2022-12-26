use crate::files::types::Files;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileDirectory;
use speedy::{Readable, Writable};
use std::collections::HashMap;
use ic_kit::Principal;
use serde::{Deserialize, Serialize};
use ic_cdk::export::candid::CandidType;
#[derive(Clone, Readable, Writable)]
pub struct User {
    pub user_name: String,
    pub address: SPrincipal,
}

#[derive(Deserialize, Serialize, Debug, Readable, Writable, CandidType)]
pub struct Profile {
    // image: SOption<SVec<u8>>,
    // username: Option<String>,
    // last_name: Option<String>,
    // first_name: Option<String>,
    // birthdate: Option<String>,
    // email: Option<String>,
    // emails: Option<Vec<String>>,
}


pub type Users = Vec<User>;

pub type UserFiles = HashMap<String, FileDirectory>;

pub type UserProfile = HashMap<SPrincipal, Profile>;
