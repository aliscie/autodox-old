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