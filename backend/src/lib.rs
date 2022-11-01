mod apis;
mod backend_error;
mod data_structure;
mod structure;

use backend_error::*;
use std::collections::HashMap;
use structure::*;

use ic_kit::{candid::export_service, macros::*};

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("backend.did"), export_candid()).expect("Write failed.");
    }
}
