use candid::CandidType;

#[derive(CandidType)]
pub enum CreateFileResponse{
    Success,
    UserNotRegisted,
}