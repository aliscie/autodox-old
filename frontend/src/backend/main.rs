use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yewdux::prelude::Dispatch;

use shared::invoke_async;
use shared::schema::FileDirectory;

pub fn initialize() -> Result<(), String> {
    spawn_local(async move {
        let x = self::on_startup().await;
        log_1(&format!("printing in spawn_local : {:?}", x).into());
    });
    Ok(())
}

async fn on_startup() -> Result<(), String> {
    let mut directories = crate::backend::get_directories().await.map_err(|e| {
        log_1(&format!("error is : {:?}", e).into());
        e
    }).unwrap_or_default();
    log_1(&format!("get_directory returned : {:?}", directories).into());
    let file_tree = Dispatch::<FileDirectory>::new();
    if directories.len() == 0 {
        // create new directory tree
        let file_directory = FileDirectory::default();
        let x = crate::backend::create_directory(&file_directory).await;
        log_1(&format!("create_directory returned : {:?}", x).into());
        // setting the file
        file_tree.set(file_directory);
    } else {
        let x = directories.remove(0);
        // setting the file
        file_tree.set(x);
    }
    Ok(())
}

pub async fn call_surreal<T, U>(command: String, args: Option<&U>) -> Result<T, String>
    where
        T: DeserializeOwned,
        U: Serialize,
{
    let x = invoke_async::<U>(command, args)
        .await
        .map_err(|e| format!("{:?}", e))?;
    from_value(x).map_err(|e| e.to_string())
}
