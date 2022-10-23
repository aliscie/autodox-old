use ic_kit::candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct ReadFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
}