use candid::CandidType;

#[derive(CandidType)]
pub enum AddChildContentResponse{
    Success,
    UserNotRegisted,
    Unauthorized,
    FileDoesNotExist,
}