pub use crate::error::Error;
use crate::{context::Context, store::Store};
use surrealdb::sql::{Array, Object, Value};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;
