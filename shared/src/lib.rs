#![allow(unused_imports)]
extern crate uuid;

use lazy_static::lazy_static;
pub use tree::Tree;

mod tree;
mod error;
pub use error::Error;
pub mod schema;
pub mod macros;
pub mod traits;

pub use macros::*;

// pub mod extensions;
cfg_if::cfg_if! {
    if #[cfg(feature = "frontend")]{
        mod invoke;
        pub use invoke::{invoke, invoke_async};
    }
}

// lazy_static! {
//     pub static ref IS_WEB: bool = {
//         #[cfg(feature = "web")] {
//             true
//         }
//         false
//     };
// }
