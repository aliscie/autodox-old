//use sea_orm::entity::prelude::*;
//use sea_orm::{TryGetError, TryGetable, Value};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default )]
pub struct UuidSet(pub IndexSet<Uuid>);

impl Deref for UuidSet {
    type Target = IndexSet<Uuid>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UuidSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IndexSet<Uuid>> for UuidSet {
    fn from(source: IndexSet<Uuid>) -> Self {
        Self(source)
    }
}

impl Into<IndexSet<Uuid>> for UuidSet {
    fn into(self) -> IndexSet<Uuid> {
        self.0
    }
}
