use surrealdb::sql::{Array, Object, Value};
use uuid::Uuid;

use crate::{context::Context, store::Store};
pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
