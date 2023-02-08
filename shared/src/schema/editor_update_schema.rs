use crate::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditorChange {
    Update(EditorElementUpdate),
    Create(EditorElementCreate),
    Delete(EditorElementDelete),
}
