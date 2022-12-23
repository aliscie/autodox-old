#![allow(unused_imports)]
extern crate uuid;

use lazy_static::lazy_static;
pub use tree::Tree;

mod error;
pub mod tree;
pub use error::Error;
pub mod macros;
pub mod schema;
pub mod traits;
pub mod id;

pub use macros::*;

// pub mod extensions;
cfg_if::cfg_if! {
    if #[cfg(feature = "frontend")]{
        mod invoke;
        pub use invoke::{invoke, invoke_async};
    }
}
