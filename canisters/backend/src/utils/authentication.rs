pub fn get_user() -> String {
    let caller: SPrincipal = SPrincipal(ic_cdk::caller());
    let users = s!(crate:users::types::Users);
    match get_username(caller, &users) {
        None => "is_longed_in".to_string(), //  CreateFileResponse::UserNotRegisted,
        Some(username) => "not_longed_in".to_string() // username,
    }
}