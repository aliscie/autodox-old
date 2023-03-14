use crate::nft::types::{NftCollection};

use ic_cdk::export::candid::{
    candid_method, check_prog, export_service, CandidType, Deserialize, IDLProg, TypeEnv,
};
mod initialize;

use initialize::*;
mod nft;

use ic_cdk_macros::*;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};

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


