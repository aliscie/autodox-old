use crate::id::Id;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    rc::Rc,
};
#[cfg(feature = "tauri")]
use surrealdb::sql::{Array, Object, Thing, Value};
use uuid::Uuid;

#[cfg(feature = "frontend")]
use {web_sys::console::log_1, yewdux::prelude::*};

use crate::{
    traits::{Creatable, Entity, GetId, Queryable, Updatable},
    Error, Tree,
};

/// marker trait of id
//pub trait InternalId : Into<Uuid>{}

//impl InternalId for Uuid {}
//impl InternalId for String {}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ElementTree {
    pub id: Id,
    pub elements: Tree<Id, EditorElement>,
}

/// make id as generic
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EditorElement {
    pub id: Id,
    pub text: String,
    pub tag: Option<String>,
    pub attrs: HashMap<Attrs, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub enum Attrs {
    #[default]
    Style,
    Href,
    Src,
}

#[cfg(not(feature = "backend"))]
impl Default for ElementTree {
    fn default() -> Self {
        let mut tree = Tree::new();
        let element = EditorElement::default();
        tree.root = Some(element.id);
        tree.vertices.insert(element.id, element);
        Self {
            id: Id::new(),
            elements: tree,
        }
    }
}

#[cfg(not(feature = "backend"))]
impl Default for EditorElement {
    fn default() -> Self {
        // this creates a root element
        Self {
            id: Id::new(),
            tag: None,
            text: "".to_owned(),
            attrs: HashMap::new(),
        }
    }
}

impl EditorElement {
    #[inline]
    pub fn new<T>(id: T, text: String, attrs: HashMap<Attrs, String>) -> Self
    where
        T: Into<Id>,
    {
        Self {
            id: id.into(),
            text,
            tag: None,
            attrs,
        }
    }
}

#[cfg(feature = "tauri")]
impl Queryable for EditorElement {}

impl GetId for EditorElement {
    type Id = Id;
    fn get_id(&self) -> Self::Id {
        self.id
    }
}

#[cfg(feature = "tauri")]
impl Entity for EditorElement {
    type DatabaseType = Object;
    fn table_name() -> String {
        "editor_element".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorElementCreate {
    pub id: Id,
    pub text: String,
    pub attrs: HashMap<Attrs, String>,
    pub tree_id: Id,
    pub parent_id: Id,
    pub children: Option<Vec<Id>>,
    // represents the element after which the current element should be pushed
    pub prev_element_id: Option<Id>,
}

/// type for updating editor elements
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorElementUpdate {
    pub id: Id,
    pub text: Option<String>,
    pub attrs: Option<HashMap<Attrs, String>>,
    pub parent: Option<Id>,
    pub children: Option<IndexSet<Id>>,
}

#[cfg(not(feature = "backend"))]
impl Default for EditorElementUpdate {
    fn default() -> Self {
        Self {
            id: Id::new(),
            text: None,
            attrs: None,
            parent: None,
            children: None,
        }
    }
}

impl From<EditorElementCreate> for EditorElement {
    fn from(v: EditorElementCreate) -> Self {
        Self {
            id: v.id,
            text: v.text,
            tag: None,
            attrs: v.attrs,
        }
    }
}

#[cfg(feature = "tauri")]
impl Entity for EditorElementCreate {
    type DatabaseType = Object;
    fn table_name() -> String {
        "editor_element".to_string()
    }
}

#[cfg(feature = "tauri")]
impl Entity for EditorElementUpdate {
    type DatabaseType = Object;
    fn table_name() -> String {
        "editor_element".to_string()
    }
}

#[cfg(feature = "tauri")]
impl Creatable for EditorElementCreate {}

#[cfg(feature = "tauri")]
impl Creatable for ElementTree {}

#[cfg(feature = "tauri")]
impl Updatable for EditorElementUpdate {}

#[cfg(feature = "tauri")]
impl Queryable for ElementTree {}

#[cfg(feature = "tauri")]
impl From<EditorElementUpdate> for Object {
    fn from(value: EditorElementUpdate) -> Self {
        let mut object = BTreeMap::new();
        let children: Vec<Value> = match value.children {
            Some(x) => x
                .into_iter()
                .map(|f| Value::Thing(Thing::from((EditorElement::table_name(), f.to_string()))))
                .collect(),
            None => Vec::new(),
        };
        if let Some(text) = value.text {
            object.insert("text".to_string(), text.into());
        }
        if let Some(attrs) = value.attrs {
            attrs_to_object(attrs, &mut object);
        }
        object.insert("children".to_string(), children.into());
        object.into()
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Object> for ElementTree {
    type Error = Error;
    fn try_from(mut value: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .remove("id")
                .ok_or(Error::XPropertyNotFound(format!(
                    "id not found in object for ElementTree"
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
            elements: value
                .remove("elements")
                .and_then(|f| -> Option<Object> { f.try_into().ok() })
                .ok_or(Error::XPropertyNotFound(format!(
                    "files not found in object for FileDirectory"
                )))?
                .try_into()?,
        })
    }
}

#[cfg(feature = "tauri")]
impl Entity for ElementTree {
    type DatabaseType = Object;
    fn table_name() -> String {
        "element_tree".into()
    }
}

#[cfg(feature = "tauri")]
fn attrs_to_object(attrs: HashMap<Attrs, String>, object: &mut BTreeMap<String, Value>) {
    for (attrs, data) in attrs {
        let attr = match attrs {
            Attrs::Src => "Src",
            Attrs::Href => "Href",
            Attrs::Style => "Style",
        };
        object.insert(attr.to_string(), data.into());
    }
}

#[cfg(feature = "tauri")]
impl From<EditorElementCreate> for Object {
    fn from(value: EditorElementCreate) -> Self {
        let children: Vec<Value> = match value.children {
            Some(x) => x
                .into_iter()
                .map(|f| Value::Thing(Thing::from((EditorElement::table_name(), f.to_string()))))
                .collect(),
            None => Vec::new(),
        };
        let mut x: BTreeMap<String, Value> = BTreeMap::from([
            ("id".into(), value.id.into()),
            ("text".into(), value.text.into()),
            ("children".into(), Array(children).into()),
        ])
        .into();
        attrs_to_object(value.attrs, &mut x);
        x.into()
    }
}

#[cfg(feature = "tauri")]
impl From<EditorElement> for Object {
    fn from(value: EditorElement) -> Self {
        let mut x = BTreeMap::from([
            ("id".into(), value.id.into()),
            ("text".into(), value.text.into()),
        ]);
        attrs_to_object(value.attrs, &mut x);
        x.into()
    }
}

#[cfg(feature = "tauri")]
impl From<ElementTree> for Object {
    fn from(value: ElementTree) -> Self {
        BTreeMap::from([
            ("id".into(), value.id.into()),
            ("elements".into(), Value::Object(value.elements.into())),
        ])
        .into()
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Object> for EditorElement {
    type Error = crate::Error;
    fn try_from(mut value: Object) -> Result<Self, Self::Error> {
        let mut attrs: HashMap<Attrs, String> = HashMap::new();
        if let Some(x) = value.remove("Src") {
            attrs.insert(
                Attrs::Src,
                x.try_into().map_err(|_| Error::XValueNotOfType("String"))?,
            );
        }
        if let Some(x) = value.remove("Href") {
            attrs.insert(
                Attrs::Href,
                x.try_into().map_err(|_| Error::XValueNotOfType("String"))?,
            );
        }
        if let Some(x) = value.remove("Style") {
            attrs.insert(
                Attrs::Style,
                x.try_into().map_err(|_| Error::XValueNotOfType("String"))?,
            );
        }
        Ok(Self {
            id: value
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
            text: value
                .remove("text")
                .ok_or(Error::XPropertyNotFound("text".into()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
            tag: None,
            attrs,
        })
    }
}

#[cfg(feature = "frontend")]
impl Store for ElementTree {
    fn new() -> Self {
        ElementTree::default()
    }
    fn should_notify(&self, old: &Self) -> bool {
        old != self
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EditorElementDelete {
    pub parent_id: Id,
    pub id: Id,
    pub tree_id: Id,
}
