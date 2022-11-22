use ic_kit::candid::{
    CandidType, Deserialize
};

use super::FileMode;

#[derive(CandidType, Deserialize)]
pub struct CreateFileData{
    pub parent_id: String,
    pub name: String,
    pub mode: FileMode,
    pub content: String,
}

#[derive(CandidType, Deserialize)]
pub struct AddChildContent{
    pub parent_id: String,
    pub child_id: String,
    pub content: String,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateFileData{
    pub parent_id: String,
    pub child_id: Option<String>,
    pub new_content: String,
}

#[derive(CandidType, Deserialize)]
pub struct DeleteFileData{
    pub parent_id: String,
    pub child_id: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct ChildContentData{
    pub parent_id: String,
    pub child_id: String,
    pub content: String,
}

#[derive(CandidType, Deserialize)]
pub struct AddViewerData{
    pub parent_id: String,
    pub viewer: String,
}

#[derive(CandidType, Deserialize)]
pub struct RemoveViewerData{
    pub parent_id: String,
    pub viewer: String,
}

#[derive(CandidType, Deserialize)]
pub struct ChangeFileModeData{
    pub parent_id: String,
    pub new_mode: FileMode
}