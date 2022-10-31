use crate::{context::Context, store::Store};
pub use crate::error::Error as Error;
use surrealdb::sql::{Array, Object, Value};
use uuid::Uuid;


pub type Result<T> = core::result::Result<T, Error>;
