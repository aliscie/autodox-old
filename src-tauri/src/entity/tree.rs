use sea_orm::entity::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::convert::{From, TryInto};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tree")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub vertices: Json,
    pub adjacency: Json,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl<ID, T> From<shared::Tree<ID, T>> for Model
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug + Serialize,
    T: PartialEq + Eq + Clone + Default + Debug + Serialize,
{
    fn from(item: shared::Tree<ID, T>) -> Self {
        Self {
            id: item.id,
            vertices: serde_json::json!(item.vertices),
            adjacency: serde_json::json!(item.adjacency),
        }
    }
}

impl<'a, ID, T> TryInto<shared::Tree<ID, T>> for Model
where
    ID: Hash + PartialEq + Eq + Clone + Default + Debug + DeserializeOwned,
    T: PartialEq + Eq + Clone + Default + Debug + DeserializeOwned,
{
    type Error = serde_json::Error;
    fn try_into(self) -> Result<shared::Tree<ID, T>, Self::Error> {
        Ok(shared::Tree {
            id: self.id,
            vertices: serde_json::from_value(self.vertices)?,
            adjacency: serde_json::from_value(self.adjacency)?,
        })
    }
}
