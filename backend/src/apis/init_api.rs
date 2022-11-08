use ic_kit::{candid::candid_method, macros::*};
use ic_stable_memory::{collections::hash_map::SHashMap, s, stable_memory_init};

use crate::schema::Storage;

#[init]
#[candid_method(init)]
pub fn init() {
    stable_memory_init(true, 0);
    s!(Storage = SHashMap::new())
}
