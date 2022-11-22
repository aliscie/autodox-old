use candid::CandidType;

#[derive(CandidType)]
pub enum UserNameResponse{
    UserNotRegisted,
    User{ user_name: String}
}