#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[cfg(feature = "backend")]
use candid::CandidType;

use serde::{Deserialize, Serialize};
use crate::id::Id;

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct UserQuery {
    pub address: String,
    pub username: Option<String>,
    pub image: Option<Vec<u8>>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub birthdate: Option<String>,
    pub email: Option<String>,
    pub emails: Option<Vec<String>>,
}


impl UserQuery {
    pub fn default() -> Self {
        Self {
            address: Id::default().to_string(),
            username: None,
            image: None,
            last_name: None,
            first_name: None,
            birthdate: None,
            email: None,
            emails: None,
        }
    }
    pub fn update<T>(&mut self, key: &str, value: T)
        where T: std::convert::Into<String> + Clone
    {
        match key {
            "first_name" => self.first_name = Some(value.clone().into()),
            "last_name" => self.last_name = Some(value.clone().into()),
            "username" => self.username = Some(value.clone().into()),
            "birthdate" => self.birthdate = Some(value.clone().into()),
            // "image" => self.image = Some(value.clone()),
            _ => println!("Invalid field"),
        }
    }
}