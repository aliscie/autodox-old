use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types::{MyStringsSlice, MyStrings};

#[update]
fn add_my_string(entry: String) {
    let mut my_strings = s!(MyStrings);

    // this call now pushes new value directly to stable memory
    my_strings.push(&entry);

    // only saves SVec's pointer, instead of the whole collection
    s! { MyStrings = my_strings }
    ;
}
