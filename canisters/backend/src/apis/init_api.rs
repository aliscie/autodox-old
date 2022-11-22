use ic_kit::{
    macros::*, candid::candid_method,
};
use ic_stable_memory::{
    s, stable_memory_init
};
// use crate::structure::Storage;

use crate::structure::*;

// #[init]
// #[candid_method(init)]
// pub fn init(){
//     stable_memory_init(true, 0);
//     s!( Storage = SHashMap::new())
// }

#[init]
#[candid_method(init)]
pub fn init(){
    stable_memory_init(true, 0);
    s! (Users = Users::new());
    s!(UserFiles = UserFiles::new());
}