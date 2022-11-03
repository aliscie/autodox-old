//! This is the main (and only for now) application Error type.
//! It's using 'this-error' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!     - By best practices, `anyhow` is not used in application code, but can be used in unit or integration test (will be in dev_dependencies when used)
//!
use serde::Serialize;

use shared::Error as SharedError;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum Error {
    #[error("Fail to get Ctx")]
    CtxFail,

    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error("Property '{0}' not found")]
    XPropertyNotFound(String),

    #[error("Fail to create. Cause: {0}")]
    StoreFailToCreate(String),

    #[error(transparent)]
    Surreal(surrealdb::Error),
    //#[error(transparent)]
    //IO(#[from] std::io::Error),
}

impl From<SharedError> for Error {
    fn from(val: SharedError) -> Self {
        match val {
            SharedError::XValueNotOfType(x) => Self::XValueNotOfType(x),
            SharedError::XPropertyNotFound(x) => Self::XPropertyNotFound(x),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        Self::Surreal(val)
    }
}
