use sea_orm::entity::prelude::*;
use sea_orm::{TryGetError, TryGetable, Value};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
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

impl From<UuidSet> for Value {
    fn from(source: UuidSet) -> Self {
        Value::String(serde_json::to_string(&source).ok().map(Box::new))
    }
}

impl TryGetable for UuidSet {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        let json_str: String = res.try_get(pre, col).map_err(TryGetError::DbErr)?;
        serde_json::from_str(&json_str).map_err(|e| TryGetError::DbErr(DbErr::Json(e.to_string())))
    }
}

impl sea_query::ValueType for UuidSet {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        match v {
            Value::String(Some(x)) => Ok(UuidSet(
                serde_json::from_str(&x).map_err(|_| sea_query::ValueTypeErr)?,
            )),
            _ => Err(sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(UuidVec).to_owned()
    }

    fn column_type() -> sea_query::ColumnType {
        sea_query::ColumnType::String(None)
    }
}
