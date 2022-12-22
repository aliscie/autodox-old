use crate::users::types::User;
use ic_stable_memory::utils::ic_types::SPrincipal;

pub fn is_registered(address: &SPrincipal, mut list: Vec<User>) -> Option<String>{
    list.retain(|user| &user.address == address);
    if list.len() == 0{
        return None
    }
    Some(list.get(0).unwrap().user_name.clone())
}

// pub fn username_check(address: &SPrincipal, mut list: Vec<User>) -> Option<String>{
//     list.retain(|user| &user.address == address);
//     if list.len() == 0{
//         return None
//     }
//     Some(list.get(0).unwrap().user_name.clone())
// }
