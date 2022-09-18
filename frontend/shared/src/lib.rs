mod tree;
use lazy_static::lazy_static;
pub use tree::Tree;
cfg_if::cfg_if!{
    if #[cfg(not(features = "web"))]{
        mod invoke;
        pub use invoke::{invoke, invoke_async};
    }
}

lazy_static!{
    pub static ref IS_WEB: bool = {
        cfg_if::cfg_if!{
            if #[cfg(features = "web")]{
                return true;
            }
            else{
                return false;
            }
        }
    };
}
