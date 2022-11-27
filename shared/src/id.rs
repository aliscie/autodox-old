#[cfg(feature = "backend")]
use candid::{
    types::{Serializer, Type},
    CandidType,
};

#[cfg(feature = "tauri")]
use surrealdb::sql::Value;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Debug};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};
use uuid::Uuid;

#[derive(
    PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct Id(pub Uuid);

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

#[cfg(feature = "tauri")]
impl TryFrom<Value> for Id {
    type Error = surrealdb::Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(Id(value.try_into()?))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Id {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(|f| Self(f))
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
