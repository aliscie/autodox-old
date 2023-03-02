
use ic_cdk::export::candid::{
    candid_method, check_prog, export_service, CandidType, Deserialize, IDLProg, TypeEnv,
};

mod elements;
mod files;
mod initialize;
mod users;


pub use elements::*;
pub use files::*;
pub use users::*;

mod utils;

use crate::files::types::MyStrings;
use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};
use initialize::*;

pub mod custom_traits;

use shared::schema::*;

// before upgrading the code of the canister (before deploying the app)
#[pre_upgrade]
fn pre_upgrade() {
    stable_memory_pre_upgrade();
}

// after upgrading the code of the canister
#[post_upgrade]
fn post_upgrade() {
    stable_memory_post_upgrade(0);
}

// #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     export_service!();
//     __export_service()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use std::fs::{create_dir_all, write};
//     use std::path::PathBuf;
//
//     #[test]
//     fn save_candid() {
//         let dir = PathBuf::from("/Users/ahmed/Desktop/autodox-tauri/canisters/backend");
//         match create_dir_all(&dir) {
//             Ok(_) => println!("Successfully created directory"),
//             Err(e) => println!("Failed to create directory: {}", e),
//         }
//
//         let res = write(dir.join("backend.did"), export_candid());
//         println!("-------- Wrote to {:?}", dir);
//         println!("-------- res {:?}", res);
//     }
// }
