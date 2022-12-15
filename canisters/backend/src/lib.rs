use ic_cdk_macros::*;
use ic_stable_memory::collections::vec::SVec;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};

type MyStrings = SVec<String>;
type MyStringsSlice = Vec<String>;

#[init]
fn init() {
    stable_memory_init(true, 0);

    // now, our stable variable will hold an SVec pointer instead of the the whole Vec as it was previously
    s! { MyStrings = MyStrings::new() };
}

#[pre_upgrade]
fn pre_upgrade() {
    stable_memory_pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    stable_memory_post_upgrade(0);
}

#[query]
fn get_my_strings_page(from: u64, to: u64) -> MyStringsSlice {
    let my_strings = s!(MyStrings);

    // our stable collection can be very big, so we only return a page of it
    let mut result = MyStringsSlice::new();

    for i in from..to {
        let entry: String = my_strings.get_cloned(i).expect(format!("No entry at pos {}", i).as_str());
        result.push(entry);
    }

    result
}

#[update]
fn add_my_string(entry: String) {
    let mut my_strings = s!(MyStrings);

    // this call now pushes new value directly to stable memory
    my_strings.push(&entry);

    // only saves SVec's pointer, instead of the whole collection
    s! { MyStrings = my_strings };
}
