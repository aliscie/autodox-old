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

use crate::{
    traits::{Creatable, Entity, GetId},
    Error, Tree,
};

/// marker trait of id
//pub trait InternalId : Into<Uuid>{}

//impl InternalId for Uuid {}
//impl InternalId for String {}

pub struct ElementTree {
    pub id: Uuid,
    pub elements: Tree<Uuid, EditorElement>,
}

/// make id as generic
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EditorElement {
    pub id: Uuid,
    pub text: String,
    pub attrs: HashMap<Attrs, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub enum Attrs {
    #[default]
    Style,
    Href,
    Src,
}

impl Default for EditorElement {
    fn default() -> Self {
        // this creates a root element
        Self {
            id: Uuid::new_v4(),
            text: "".to_owned(),
            attrs: HashMap::new(),
        }
    }
}

impl EditorElement {
    #[inline]
    pub fn new(
        id: Uuid,
        text: String,
        attrs: HashMap<Attrs, String>,
    ) -> Self {
        Self {
            id,
            text,
            attrs,
        }
    }
}

impl GetId for EditorElement{
    type Id = Uuid;
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

#[derive(Serialize, Deserialize)]
pub struct EditorElementCreate {
    pub id: Uuid,
    pub text: String,
    pub attrs: HashMap<Attrs, String>,
    pub file_id: Uuid,
    pub parent: Option<Uuid>,
}

/// type for updating editor elements
#[derive(Serialize, Deserialize)]
pub struct EditorElementUpdate {
    pub text: Option<String>,
    pub attrs: Option<HashMap<Attrs, String>>,
    pub parent: Option<Uuid>,
    pub children: Option<IndexSet<Uuid>>,
}

#[cfg(feature = "tauri")]
impl Entity for EditorElementCreate {
    type DatabaseType = Object;
    fn table_name() -> String {
        "editor_element".to_string()
    }
}

#[cfg(feature = "tauri")]
fn attrs_to_object(attrs: HashMap<Attrs, String>, x : &mut BTreeMap<String, Value>){
    for (attrs, data) in attrs {
        let attr = match attrs {
            Attrs::Src => "Src",
            Attrs::Href => "Href",
            Attrs::Style => "Style",
        };
        x.insert(attr.to_string(), data.into());
    }
}

#[cfg(feature = "tauri")]
impl From<EditorElementCreate> for Object {
    fn from(value: EditorElementCreate) -> Self {
        let mut x: BTreeMap<String, Value> = BTreeMap::from([
            ("id".into(), value.id.into()),
            ("text".into(), value.text.into()),
            ("children".into(), Array::new().into()),
        ])
        .into();
        attrs_to_object(value.attrs, &mut x);
        match value.parent {
            Some(u) => x.insert(
                "parent".to_owned(),
                Thing::from((EditorElement::table_name(), u.to_string())).into(),
            ),
            None => x.insert("parent".to_owned(), Value::None),
        };
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
                .ok_or(Error::XPropertyNotFound("uuid not found".into()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("Uuid"))?,
            text: value
                .remove("text")
                .ok_or(Error::XPropertyNotFound("text".into()))?
                .try_into()
                .map_err(|_| Error::XValueNotOfType("String"))?,
            attrs,
        })
    }
}
