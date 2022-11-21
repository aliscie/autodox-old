use ic_kit::macros::*;
use ic_stable_memory::{stable_memory_pre_upgrade, stable_memory_post_upgrade};

#[pre_upgrade]
pub fn pre_upgrade() {
    stable_memory_pre_upgrade()
}

#[post_upgrade]
pub fn post_upgrade() {
    stable_memory_post_upgrade(0)
}