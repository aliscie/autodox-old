mod error;
use error::*;

mod api;

mod structure;
use structure::*;

use ic_kit::{
    Principal,
    macros::*,
    candid::export_service
};

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("storage.did"), export_candid()).expect("Write failed.");
    }
}