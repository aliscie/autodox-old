use crate::{
    id::Id,
    traits::{Creatable, Entity, Queryable, Updatable},
    Error, Tree,
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

#[cfg(feature = "backend")]
use candid::CandidType;

#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub enum FileMode {
    Public,
    Private,
    Restricted,
}

//#[cfg(feature = "backend")]
//impl CandidType for FileMode {
//    fn _ty() -> candid::types::Type {
//        candid::types::Type::Variant(<[_]>::into_vec(Box::new([
//            candid::types::Field {
//                id: candid::types::Label::Named("Public".to_owned()),
//                ty: candid::types::Type::Null,
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("Private".to_owned()),
//                ty: candid::types::Type::Null,
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("Restricted".to_owned()),
//                ty: candid::types::Type::Null,
//            },
//        ])))
//    }
//
//    fn id() -> candid::types::TypeId {
//        candid::types::TypeId::of::<FileMode>()
//    }
//
//    fn idl_serialize<S>(&self, _serializer: S) -> std::result::Result<(), S::Error>
//    where
//        S: candid::types::Serializer,
//    {
//        match *self {
//            FileMode::Private => {
//                let mut _ser = _serializer.serialize_variant(0_64)?;
//            }
//            FileMode::Public => {
//                let mut _ser = _serializer.serialize_variant(1_u64)?;
//            }
//            FileMode::Restricted => {
//                let mut _ser = _serializer.serialize_variant(2u64)?;
//            }
//        };
//        Ok(())
//    }
//}

/// type for creating file
#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "backend", derive(Readable, Writable, CandidType))]
pub struct FileNodeCreate {
    pub id: Id,
    pub name: String,
    pub directory_id: Id,
    pub parent_id: Id,
    pub children: Option<Vec<Id>>,
}

impl From<FileNodeCreate> for FileNode {
    fn from(value: FileNodeCreate) -> Self {
        Self {
            id: value.id,
            name: value.name,
            element_tree: None,
        }
    }
}

//#[cfg(feature = "backend")]
//impl CandidType for FileNodeCreate {
//    fn _ty() -> candid::types::Type {
//        candid::types::Type::Record(<[_]>::into_vec(Box::new([
//            candid::types::Field {
//                id: candid::types::Label::Named("id".to_string()),
//                ty: <Id as CandidType>::ty(),
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("name".to_string()),
//                ty: <String as CandidType>::ty(),
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("mode".to_string()),
//                ty: <FileMode as CandidType>::ty(),
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("directory_id".to_string()),
//                ty: <Id as CandidType>::ty(),
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("parent_id".to_string()),
//                ty: <Id as CandidType>::ty(),
//            },
//            candid::types::Field {
//                id: candid::types::Label::Named("children".to_string()),
//                ty: <Option<Vec<Id>> as CandidType>::ty(),
//            },
//        ])))
//    }
//
//    fn id() -> candid::types::TypeId {
//        candid::types::TypeId::of::<FileNodeCreate>()
//    }
//
//    fn idl_serialize<S>(&self, _serializer: S) -> std::result::Result<(), S::Error>
//    where
//        S: candid::types::Serializer,
//    {
//        let mut ser = _serializer.serialize_struct()?;
//        candid::types::Compound::serialize_element(&mut ser, &self.id)?;
//        candid::types::Compound::serialize_element(&mut ser, &self.name)?;
//        candid::types::Compound::serialize_element(&mut ser, &self.mode)?;
//        candid::types::Compound::serialize_element(&mut ser, &self.directory_id)?;
//        candid::types::Compound::serialize_element(&mut ser, &self.parent_id)?;
//        candid::types::Compound::serialize_element(&mut ser, &self.children)?;
//        Ok(())
//    }
//}

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
#[cfg_attr(feature = "backend", derive(Readable, Writable))]
pub struct FileNodeUpdate {
    pub id: Id,
    pub children: Option<Vec<Id>>,
    // TODO : cannot update this using this method think of something else
    pub parent_id: Option<Id>,
    pub name: Option<String>,
    pub element_tree: Option<Id>,
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
        if let Some(element_tree_id) = value.element_tree {
            object.insert(
                ElementTree::table_name(),
                Thing::from((ElementTree::table_name(), element_tree_id.0.to_string())).into(),
            );
        }
        Object(object)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "backend", derive(Readable, Writable))]
pub struct FileNode {
    pub id: Id,
    pub name: String,
    pub element_tree: Option<Id>,
}

#[cfg(feature = "backend")]
impl candid::types::CandidType for FileNode {
    fn _ty() -> ::candid::types::Type {
        candid::types::Type::Record(<[_]>::into_vec(Box::new([
            candid::types::Field {
                id: candid::types::Label::Named("id".to_string()),
                ty: <Id as candid::types::CandidType>::ty(),
            },
            candid::types::Field {
                id: candid::types::Label::Named("name".to_string()),
                ty: <String as candid::types::CandidType>::ty(),
            },
            candid::types::Field {
                id: candid::types::Label::Named("element_tree".to_string()),
                ty: <Option<Id> as ::candid::types::CandidType>::ty(),
            },
        ])))
    }
    fn id() -> candid::types::TypeId {
        candid::types::TypeId::of::<FileNode>()
    }
    fn idl_serialize<__S>(&self, __serializer: __S) -> std::result::Result<(), __S::Error>
    where
        __S: candid::types::Serializer,
    {
        let mut ser = __serializer.serialize_struct()?;
        candid::types::Compound::serialize_element(&mut ser, &self.id)?;
        candid::types::Compound::serialize_element(&mut ser, &self.name)?;
        candid::types::Compound::serialize_element(&mut ser, &self.element_tree)?;
        Ok(())
    }
}

#[cfg(not(feature = "backend"))]
impl Default for FileNode {
    fn default() -> Self {
        Self {
            id: Id::new(),
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

#[cfg(feature = "tauri")]
impl Queryable for FileNode {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[cfg_attr(feature = "backend", derive(Readable, Writable))]
pub struct FileDirectory {
    pub id: Id,
    pub name: String,
    pub files: Tree<Id, FileNode>,
}



#[cfg(feature = "backend")]
impl candid::types::CandidType for FileDirectory {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Record(<[_]>::into_vec(Box::new([
            candid::types::Field {
                id: candid::types::Label::Named("id".to_string()),
                ty: <Id as candid::types::CandidType>::ty(),
            },
            candid::types::Field {
                id: candid::types::Label::Named("files".to_string()),
                ty: <Tree<Id, FileNode> as candid::types::CandidType>::ty(),
            },
            candid::types::Field {
                id: candid::types::Label::Named("name".to_string()),
                ty: <String as candid::types::CandidType>::ty(),
            },
        ])))
    }
    fn id() -> candid::types::TypeId {
        candid::types::TypeId::of::<FileDirectory>()
    }
    fn idl_serialize<__S>(&self, __serializer: __S) -> std::result::Result<(), __S::Error>
    where
        __S: candid::types::Serializer,
    {
        let mut ser = __serializer.serialize_struct()?;
        candid::types::Compound::serialize_element(&mut ser, &self.id)?;
        candid::types::Compound::serialize_element(&mut ser, &self.files)?;
        candid::types::Compound::serialize_element(&mut ser, &self.name)?;
        Ok(())
    }
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

#[cfg(not(feature = "backend"))]
impl Default for FileDirectory {
    fn default() -> Self {
        let mut d = Self::new(Id::new(), "default".to_string());
        let id = Id::new();
        d.files.push_vertex(
            id.into(),
            FileNode {
                id: id.into(),
                name: "root".into(),
                element_tree: None,
            },
        );
        d.files.adjacency.insert(id.clone().into(), Vec::new());
        d.files.root = Some(id.into());
        return d;
    }
}

impl FileDirectory {
    #[inline]
    pub fn new(id: Id, name: String) -> Self {
        Self {
            files: Tree::new(),
            id: id.into(),
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
            (
                "element_tree".into(),
                val.element_tree.map_or(Value::None, |id| {
                    Value::Thing(Thing::from((ElementTree::table_name(), id.to_string())))
                }),
            ),
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
            element_tree: object.remove("element_tree").map_or(None, |f| match f {
                Value::Thing(x) => x.id.to_raw().as_str().parse::<Uuid>().map(Id::from).ok(),
                _ => None,
            }),
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FileNodeDelete {
    pub id: Id,
    pub tree_id: Id,
    pub parent_id: Id,
}
