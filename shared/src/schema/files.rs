use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq)]
pub struct FileNode {
    pub id: Uuid,
    pub name: String,
    pub element_tree_id: Option<Uuid>,
    // skipping it now later this will be removed
    #[serde(skip)]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq)]
pub struct FileDirectory {
    pub id: Uuid,
    pub name: String,
    pub root: Option<Uuid>,
}
