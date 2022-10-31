use yewdux::prelude::*;
#[derive(Debug, PartialEq, Eq, Clone, Store)]
pub struct DeviceInfo {
    pub online: bool,
    pub web: bool,
    pub auth: bool,
    // could add some other fields here!
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            online: false,
            web: *crate::IS_WEB,
            auth: false,
        }
    }
}
