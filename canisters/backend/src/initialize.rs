use ic_cdk_macros::*;
use ic_stable_memory::{s, stable_memory_init, stable_memory_post_upgrade, stable_memory_pre_upgrade};
use crate::files::types::{MyStrings};
use crate::users::types::{Users, UserFiles, UserProfile};


#[init]
fn init() {
    stable_memory_init(true, 0);

    // now, our stable variable will hold an SVec pointer instead of the the whole Vec as it was previously
    s! { MyStrings = MyStrings::new() }
    s!(Users = Users::new());
    s!(UserFiles = UserFiles::new());
    s!(UserProfile = UserProfile::new());
}