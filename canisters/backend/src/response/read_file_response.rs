use std::collections::HashMap;

use candid::CandidType;

use crate::structure::ChildContent;

#[derive(CandidType)]
pub enum ReadFileResponse{
    File{
        parent_id: String,
        name: String,
        content: String,
        child_contents: HashMap<String, ChildContent>
    },
    FileDoesNotExist,
    NotAllowed,
    UserNotRegisted,
}

impl ReadFileResponse{
    pub fn generate_file_response(parent_id: String, name: String, content: String, child_contents: HashMap<String, ChildContent>) -> Self{
        Self::File { 
            parent_id, 
            name, 
            content,
            child_contents,
        }
    }
}