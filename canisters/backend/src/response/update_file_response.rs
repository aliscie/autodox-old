use candid::CandidType;

#[derive(CandidType)]
pub enum UpdateFileResponse{
    Success,
    UserNotRegisted,
    FileDoesNotExist,
    Unauthorized,
}