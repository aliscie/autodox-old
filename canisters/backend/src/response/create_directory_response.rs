use candid::CandidType;

#[derive(CandidType)]
pub enum CreateDirectoryResponse{
    Success,
    AlreadyExist,
    UserNotRegisted,
}