use crate::{
    traits::{Creatable, Entity, Queryable, Updatable},
    Error, Tree, id::Id,
};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(feature = "tauri")]
use surrealdb::sql::{Array, Object, Thing, Value};
use uuid::Uuid;
#[cfg(feature = "frontend")]
use yewdux::store::Store;

use super::{EditorElement, ElementTree};

/// type for creating file
#[derive(Deserialize, Serialize, Debug)]
pub struct FileNodeCreate {
    pub id: Uuid,
    pub name: String,
    pub directory_id: Uuid,
    pub parent_id: Uuid,
    pub children: Option<IndexSet<Uuid>>,
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
        let children: Vec<Value> = match val.children {
            Some(x) => x
                .into_iter()
                .map(|f| Value::Thing(Thing::from((FileNode::table_name(), f.to_string()))))
                .collect(),
            None => Vec::new(),
        };
        BTreeMap::from([
            ("name".into(), val.name.into()),
            ("id".into(), val.id.into()),
            ("children".into(), children.into()),
        ])
        .into()
    }
}

/// type for updating file_node
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct FileNodeUpdate {
    pub children: Option<IndexSet<Uuid>>,
    // TODO : cannot update this using this method think of something else
    pub parent_id: Option<Uuid>,
    pub name: Option<String>,
    pub element_tree: Option<Uuid>,
}

#[cfg(feature = "tauri")]
impl Entity for FileNodeUpdate {
    type DatabaseType = Object;
    fn table_name() -> String {
        "file_node".into()
    }
}

#[cfg(feature = "tauri")]
impl Updatable for FileNodeUpdate {}

#[cfg(feature = "tauri")]
impl From<FileNodeUpdate> for Object {
    fn from(value: FileNodeUpdate) -> Self {
        let mut object = BTreeMap::new();
        if let Some(children) = value.children {
            object.insert(
                "children".into(),
                Array(
                    children
                        .into_iter()
                        .map(|f| -> Value {
                            Thing::from((FileNode::table_name(), f.to_string())).into()
                        })
                        .collect(),
                )
                .into(),
            );
        }
        if let Some(name) = value.name {
            object.insert("name".to_owned(), name.into());
        }
        if let Some(element_tree) = value.element_tree {
            object.insert(
                "element_tree".into(),
                Thing::from((ElementTree::table_name(), element_tree.to_string())).into(),
            );
        }
        Object(object)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct FileNode {
    pub id: Id,
    pub name: String,
    pub element_tree: Option<Uuid>,
}

impl Default for FileNode {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().into(),
            name: "untitled".to_string(),
            element_tree: None,
        }
    }
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
    pub id: Id,
    pub name: String,
    pub files: Tree<Uuid, FileNode>,
}

#[cfg(feature = "tauri")]
impl Entity for FileDirectory {
    type DatabaseType = Object;
    fn table_name() -> String {
        "file_directory".to_string()
    }
}

#[cfg(feature = "tauri")]
impl Creatable for FileDirectory {}

#[cfg(feature = "tauri")]
impl Queryable for FileDirectory {}

impl Default for FileDirectory {
    fn default() -> Self {
        let mut d = Self::new(Uuid::new_v4(), "default".to_string());
        let id = Uuid::new_v4();
        d.files.push_vertex(
            id,
            FileNode {
                id : id.into(),
                name: "root".into(),
                element_tree: None,
            },
        );
        d.files.adjacency.insert(id.clone(), IndexSet::new());
        d.files.root = Some(id);
        return d;
    }
}

impl FileDirectory {
    #[inline]
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            files: Tree::new(),
            id : id.into(),
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
                // convert value into a id type
                .record()
                .ok_or(Error::XValueNotOfType("id not of type surrealdb::Thing"))?
                // get the actual id
                .id
                // converting into string
                .to_raw()
                .as_str()
                // into uuid
                .try_into()
                .map_err(|_| Error::XValueNotOfType("uuid"))?,
            name: object
                .remove("name")
                .ok_or(Error::XPropertyNotFound("name".to_string()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
            ..Default::default()
        })
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Object> for FileDirectory {
    type Error = crate::Error;
    fn try_from(mut value: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .remove("id")
                .ok_or(Error::XPropertyNotFound(format!(
                    "id not found in object for FileDirectory"
                )))?
                // convert value into a id type
                .record()
                .ok_or(Error::XValueNotOfType("id not of type surrealdb::Thing"))?
                // get the actual id
                .id
                // converting into string
                .to_raw()
                .as_str()
                // into uuid
                .try_into()
                .map_err(|_| Error::XValueNotOfType("uuid"))?,
            name: value
                .remove("name")
                .ok_or(Error::XPropertyNotFound(format!(
                    "name not found in object for FileDirectory"
                )))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
            files: value
                .remove("files")
                .and_then(|f| -> Option<Object> { f.try_into().ok() })
                .ok_or(Error::XPropertyNotFound(format!(
                    "files not found in object for FileDirectory"
                )))?
                .try_into()?,
        })
    }
}
