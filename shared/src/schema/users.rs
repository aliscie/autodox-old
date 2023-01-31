#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[cfg(feature = "backend")]
use candid::CandidType;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct UserQuery {
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
            username: None,
            image: None,
            last_name: None,
            first_name: None,
            birthdate: None,
            email: None,
            emails: None,
        }
    }
}