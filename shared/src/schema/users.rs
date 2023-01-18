#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[cfg(feature = "backend")]
use candid::CandidType;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct UserQuery {
    #[serde(default)]
    pub username: Option<String>,
    pub image: Option<Vec<u8>>,
    // last_name: Option<String>,
    // first_name: Option<String>,
    // birthdate: Option<String>,
    // email: Option<String>,
    // emails: Option<Vec<String>>,
}
