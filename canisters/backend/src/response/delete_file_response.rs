use candid::CandidType;

#[derive(CandidType)]
pub enum DeleteFileResponse{
    Success,
    UserNotRegisted,
    FileDoesNotExist,
    Unauthorized,
}