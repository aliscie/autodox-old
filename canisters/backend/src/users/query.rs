// use ic_stable_memory::s;
// use ic_stable_memory::utils::ic_types::SPrincipal;
// use crate::users::types::Profile;
//
// #[query]
// #[candid_method(query)]
// pub fn get_profile() -> Profile {
//     let caller = SPrincipal(ic_cdk::caller());
//     let mut profile = s!(UserProfile);
//     let user = profile.get(caller);
//     // users.push(new_user);
//     s! { UserProfile = profile};
//     return user
// }