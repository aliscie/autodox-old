use crate::files::types::Files;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::utils::ic_types::SPrincipal;
use shared::schema::FileDirectory;
use speedy::{Readable, Writable};
use std::collections::HashMap;
use ic_kit::Principal;
use serde::{Serialize};
// use ic_cdk::export::candid::CandidType;
use ic_stable_memory::s;
use shared::Error;

// #[derive(Clone, Readable, Writable)]
// pub struct User {
//     pub user_name: String,
//     pub address: SPrincipal,
// }

use candid::{CandidType, Deserialize};

#[derive(Clone, PartialEq, Readable, Writable, Deserialize, Debug, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Eq, Readable, Writable, CandidType))]
pub struct User {
    pub address: SPrincipal,
    pub image: Option<Vec<u8>>,
    pub user_name: Option<String>,
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
                return user.user_name.clone();
            }
        }
        None
    }
    // pub(crate) fn is_registered() -> bool {
    //     let curr = Self::current();
    //     if curr.is_none() {
    //         return false;
    //     }
    //
    //     let mut users = s!(Users);
    //     for user in users.iter() {
    //         if &user.address == &curr.unwrap().address { return true; }
    //     }
    //     false
    // }

    pub(crate) fn new() -> Option<Self> {
        let address = SPrincipal(ic_cdk::caller());
        if Principal::anonymous().to_string() == address.to_string() {
            return None;
        }
        Some(Self { address, image: None, user_name: None })
    }

    pub(crate) fn current() -> Option<Self> {
        let address = SPrincipal(ic_cdk::caller());
        let users = s!(Users);
        let user_name = match Self::get_username(address, &users) {
            None => return None, // User does not exists
            Some(username) => Some(username),
        };
        Some(Self { address, image: None, user_name })
    }
}

pub type Users = Vec<User>;

pub type UserFiles = HashMap<User, FileDirectory>;