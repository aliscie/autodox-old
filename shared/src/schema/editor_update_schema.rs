use crate::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use {
    candid::types::Compound,
    candid::types::Type,
    candid::CandidType,
    speedy::{Readable, Writable},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub enum EditorChange {
    Update(EditorElementUpdate),
    Create(EditorElementCreate),
    Delete(EditorElementDelete),
}
