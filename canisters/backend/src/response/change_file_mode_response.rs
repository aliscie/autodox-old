use candid::CandidType;

#[derive(CandidType)]
pub enum ChangeFileModeResponse{
    Unauthorized,
    UserNotRegisted,
    FileDoesNotExist,
    Success,
}