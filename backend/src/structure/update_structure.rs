use ic_kit::candid::{
    CandidType, Deserialize
};

#[derive(CandidType, Deserialize)]
pub struct CreateFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
    pub name: String,
    pub content: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
    pub old_text: String,
    pub new_text: String,
}

#[derive(CandidType, Deserialize)]
pub struct DeleteFileData{
    pub parent_id: Option<String>,
    pub child_id: String,
}