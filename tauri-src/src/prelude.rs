pub use surrealdb::sql::{Array, Object, Value};
pub use uuid::Uuid;
pub use shared::id::Id;

pub use crate::{context::Context, store::Store};
pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
