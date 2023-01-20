use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use shared::invoke_async;
use shared::schema::FileDirectory;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yewdux::prelude::Dispatch;

pub async fn call_surreal<T, U>(command: String, args: Option<&U>) -> Result<T, String>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let new_db_obj = invoke_async::<U>(command, args)
        .await
        .map_err(|e| format!("{:?}", e))?;
    from_value(new_db_obj).map_err(|e| e.to_string())
}
