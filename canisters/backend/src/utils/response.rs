use serde::{Serialize, Deserialize};

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, CandidType)]
// #[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
#[derive(CandidType)]
pub struct UpdateResponse {
    pub(crate) status: Status,
    pub(crate) message: String,
}

use candid::CandidType;

#[derive(CandidType)]
pub enum Status {
    Success,
    UnAuthorized,
    InvalidInput,
}