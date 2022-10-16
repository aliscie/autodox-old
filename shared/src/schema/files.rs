use crate::Tree;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "frontend")]
use yewdux::store::Store;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq)]
pub struct FileNode {
    pub id: Uuid,
    pub name: String,
    pub element_tree_id: Option<Uuid>,
    // skipping it now later this will be removed
    #[serde(skip)]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct FileDirectory {
    pub id: Uuid,
    pub name: String,
    pub files: Tree<Uuid, FileNode>,
}


impl Default for FileDirectory {
    fn default() -> Self {
        let mut d = Self::new(Uuid::new_v4(), "default".to_string());
        let id = Uuid::new_v4();
        d.files.push_vertex(
            id,
            FileNode {
                id,
                name: "root".into(),
                element_tree_id: None,
                data: "".into(),
            },
        );
        d.files.root = Some(id);
        return d;
    }
}

impl FileDirectory {
    #[inline]
    pub fn new(id : Uuid, name : String) -> Self {
        Self { files: Tree::new(), id, name }
    }
}

#[cfg(feature = "frontend")]
impl Store for FileDirectory {
    fn new() -> Self {
        Self::default() 
    }
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}
