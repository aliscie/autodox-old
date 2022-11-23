#[cfg(feature = "backend")]
use candid::{
    types::{Serializer, Type},
    CandidType,
};

#[cfg(feature = "tauri")]
use surrealdb::sql::Value;

use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl TryFrom<&str> for Id {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}

impl Deref for Id {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "tauri")]
impl From<Id> for Value {
    fn from(x: Id) -> Self {
        x.0.into()
    }
}

#[cfg(feature = "backend")]
impl CandidType for Id {
    fn _ty() -> Type {
        Type::Text
    }
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_text(&self.0.to_string())
    }
}
