use candid::CandidType;

#[derive(CandidType)]
pub enum AddViewerResponse{
    Success,
    InvalidUser,
    Unauthorized,
    AlreadyExist,
    FileDoesNotExist,
    UserNotRegisted,
}