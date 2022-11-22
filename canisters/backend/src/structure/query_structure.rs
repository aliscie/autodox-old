use ic_kit::candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct ReadFileData{
    pub parent_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct ReadContentData{
    pub parent_id: String,
    pub child_id: Option<String>,
}