use ic_kit::{
    macros::*, candid::candid_method,
};
use ic_stable_memory::{
    s, stable_memory_init, collections::hash_map::SHashMap
};
use crate::structure::Storage;

#[init]
#[candid_method(init)]
pub fn init(){
    stable_memory_init(true, 0);
    s!( Storage = SHashMap::new())
}