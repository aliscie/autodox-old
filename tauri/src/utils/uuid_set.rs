use sea_orm::entity::prelude::*;
//use sea_orm::{TryGetError, TryGetable, Value};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, FromJsonQueryResult)]
pub struct UuidSet(pub HashSet<Uuid>);

impl Deref for UuidSet {
    type Target = HashSet<Uuid>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UuidSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashSet<Uuid>> for UuidSet {
    fn from(source: HashSet<Uuid>) -> Self {
        Self(source)
    }
}

impl Into<HashSet<Uuid>> for UuidSet {
    fn into(self) -> HashSet<Uuid> {
        self.0
    }
}

