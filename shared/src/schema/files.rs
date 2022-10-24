use crate::{
    traits::{Creatable, Entity, Queryable},
    Tree,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "tauri")]
use surrealdb::sql::{Object, Value};
use uuid::Uuid;
#[cfg(feature = "frontend")]
use yewdux::store::Store;

/// type for creating file
pub struct FileNodeCreate {
    name: String,
}

#[cfg(feature = "tauri")]
impl Entity for FileNodeCreate {
    type DatabaseType = Object;
    fn table_name() -> String {
        "file_node".to_string()
    }
}

#[cfg(feature = "tauri")]
impl Creatable for FileNodeCreate {}

#[cfg(feature = "tauri")]
impl From<FileNodeCreate> for Object {
    fn from(val: FileNodeCreate) -> Self {
        BTreeMap::from([
            ("name".into(), val.name.into()),
            ("id".into(), surrealdb::sql::Uuid::new().into()),
        ])
        .into()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq)]
pub struct FileNode {
    pub id: Uuid,
    pub name: String,
    pub element_tree_id: Option<Uuid>,
    // skipping it now later this will be removed
    #[serde(skip)]
    pub data: String,
}

#[cfg(feature = "tauri")]
impl Entity for FileNode {
    type DatabaseType = Object;
    fn table_name() -> String {
        "file_node".to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct FileDirectory {
    pub id: Uuid,
    pub name: String,
    pub files: Tree<Uuid, FileNode>,
}

#[cfg(feature = "tauri")]
impl Entity for FileDirectory {
    type DatabaseType = Object;
    fn table_name() -> String {
        "file_directoyr".to_string()
    }
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
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            files: Tree::new(),
            id,
            name,
        }
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

#[cfg(feature = "tauri")]
impl From<FileDirectory> for Object {
    fn from(val: FileDirectory) -> Self {
        let mut map = BTreeMap::new();
        map.insert("id".to_owned(), Value::Uuid(surrealdb::sql::Uuid(val.id)));
        map.insert(
            "name".to_owned(),
            Value::Strand(surrealdb::sql::Strand(val.name)),
        );
        map.insert("files".to_owned(), Value::Object(val.files.into()));
        return map.into();
    }
}

#[cfg(feature = "tauri")]
impl From<FileNode> for Object {
    fn from(val: FileNode) -> Self {
        let mut map = BTreeMap::new();
        map.insert("id".to_owned(), Value::Uuid(surrealdb::sql::Uuid(val.id)));
        map.insert(
            "name".to_owned(),
            Value::Strand(surrealdb::sql::Strand(val.name)),
        );
        map.insert(
            "data".to_owned(),
            Value::Strand(surrealdb::sql::Strand(val.data)),
        );
        return map.into();
    }
}
