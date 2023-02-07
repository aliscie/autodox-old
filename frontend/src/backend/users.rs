use crate::backend;
use crate::utils::DeviceInfo;
use shared::log;
use shared::schema::UserQuery;
use yewdux::prelude::Dispatch;

pub async fn get_users() -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let res = backend::call_ic_np("get_users".to_string()).await;
        log!(&res);
        let users = serde_wasm_bindgen::from_value::<Vec<UserQuery>>(res);
        log!(&users);
        return Ok(());
    } else {
        return Err("Desktop version is not ready".to_string());
    }
}
