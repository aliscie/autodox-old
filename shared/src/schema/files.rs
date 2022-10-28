use crate::{
    traits::{Creatable, Entity, Queryable},
    Error, Tree,
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
        BTreeMap::from([
            ("id".into(), val.id.into()),
            ("name".into(), val.name.into()),
            ("files".into(), Value::Object(val.files.into())),
        ])
        .into()
    }
}

#[cfg(feature = "tauri")]
impl From<FileNode> for Object {
    fn from(val: FileNode) -> Self {
        BTreeMap::from([
            ("id".into(), val.id.into()),
            ("name".into(), val.name.into()),
            ("data".into(), val.data.into()),
        ])
        .into()
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Object> for FileNode {
    type Error = crate::Error;
    fn try_from(mut object: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: object
                .remove("id")
                .ok_or(Error::XPropertyNotFound("id".to_string()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("uuid"))?,
            name: object
                .remove("name")
                .ok_or(Error::XPropertyNotFound("name".to_string()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
            element_tree_id: None,
            data: object
                .remove("data")
                .ok_or(Error::XPropertyNotFound("data".to_string()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
        })
    }
}
