use candid::CandidType;

#[derive(CandidType)]
pub enum RegisterResponse{
    Success{user_name: String},
    AlreadyRegistered{user_name: String},
    UserNameAlreadyInUse,
}