use crate::files::types::Files;
use ic_kit::Principal;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileDirectory;
use speedy::{Readable, Writable};
use std::collections::HashMap;
// use ic_cdk::export::candid::CandidType;
use ic_stable_memory::s;
use shared::Error;

// #[derive(Clone, Readable, Writable)]
// pub struct User {
//     pub username: String,
//     pub address: SPrincipal,
// }

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Readable, Writable, Deserialize, Debug, Eq, Hash)]
pub struct User {
    pub address: SPrincipal,
    pub image: Option<Vec<u8>>,
    pub username: Option<String>,
    // last_name: Option<String>,
    // first_name: Option<String>,
    // birthdate: Option<String>,
    // email: Option<String>,
    // emails: Option<Vec<String>>,
}

impl User {
    pub fn get_username(address: SPrincipal, users: &Vec<User>) -> Option<String> {
        for user in users {
            if user.address == address {
                return user.username.clone();
            }
        }
        None
    }
    pub(crate) fn is_registered() -> bool {
        let address = SPrincipal(ic_cdk::caller());
        let mut users = s!(Users);
        for user in users.iter() {
            if &user.address.to_string() == &address.to_string() {
                return true;
            }
        }
        false
    }
    pub(crate) fn is_anonymous() -> bool {
        let address = SPrincipal(ic_cdk::caller());
        if Principal::anonymous().to_string() == address.to_string() {
            return true;
        }
        false
    }

    pub(crate) fn new() -> Option<Self> {
        let address = SPrincipal(ic_cdk::caller());
        if Principal::anonymous().to_string() == address.to_string() {
            return None;
        }
        Some(Self {
            address,
            image: None,
            username: None,
        })
    }

    pub(crate) fn current() -> Option<Self> {
        let address = SPrincipal(ic_cdk::caller());
        let mut users = s!(Users);
        for user in users.iter() {
            if &user.address.to_string() == &address.to_string() {
                return Some(Self { address, image: None, username: None });
            }
        }
        None
    }

    pub(crate) fn caller() -> SPrincipal {
        SPrincipal(ic_cdk::caller())
    }
}

pub type Users = Vec<User>;
pub type UserFiles = HashMap<User, FileDirectory>;
