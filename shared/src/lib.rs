mod tree;

use lazy_static::lazy_static;
pub use tree::Tree;
cfg_if::cfg_if! {
    if #[cfg(feature = "autodox_assets")]{
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
