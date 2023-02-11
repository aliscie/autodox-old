use candid::CandidType;
use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};

#[derive(Debug, Serialize, Deserialize, Readable, Writable, CandidType)]
pub struct UpdateResponse {
    status: Status,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Readable, Writable, CandidType)]
pub enum Status {
    Success,
    UnAuthorized,
    InvalidInput,
}
