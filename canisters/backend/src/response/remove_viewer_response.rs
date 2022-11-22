use candid::CandidType;

#[derive(CandidType)]
pub enum RemoveViewerResponse{
    Success,
    InvalidUser,
    Unauthorized,
    ViewerNotFound,
    FileDoesNotExist,
    UserNotRegisted,
}