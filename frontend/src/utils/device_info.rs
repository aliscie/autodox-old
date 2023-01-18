use shared::schema::UserQuery;
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Store)]
pub struct DeviceInfo {
    pub is_online: bool,
    pub is_web: bool,
    pub is_authed: bool,
    pub is_aside: bool,
    pub is_light_mode: bool,
    pub profile: UserQuery,
    // could add some other fields here!
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            is_online: false,
            is_web: cfg!(feature = "web"), // TODO reduce_mut(|state| state.web = invoke("is_tauri"))
            is_authed: false,
            is_aside: false,
            is_light_mode: false,
            profile: UserQuery {
                image: None,
                username: None,
            },
        }
    }
}
