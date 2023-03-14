
use crate::nft::NftCollection;
use ic_cdk_macros::*;
use ic_stable_memory::{
    s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade,
};

#[init]
pub fn init() {
    stable_memory_init(true, 0);

    s!(NftCollection = NftCollection::new());
}
