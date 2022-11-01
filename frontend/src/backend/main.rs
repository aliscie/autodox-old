use crate::utils::FileTree;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use shared::invoke_async;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yewdux::prelude::Dispatch;

pub fn initialize() -> Result<(), String> {
    spawn_local(async move {
        let x = self::on_startup().await;
        log_1(&format!("{:?}", x).into());
    });
    Ok(())
}

async fn on_startup() -> Result<(), String> {
    let directoies = crate::backend::get_directories().await.unwrap_or_default();
    log_1(&format!("{:?}", directoies).into());
    let file_tree = Dispatch::<FileTree>::new();
    if directoies.len() == 0 {
        // create new directory tree
        let x = crate::backend::create_directory("default".to_string()).await?;
        let directory = crate::backend::get_directory(x.id).await?;
        // setting the file
        //file_tree.set(FileTree { files: directory });
    } else {
        let x = directoies.get(0).unwrap();
        let directory = crate::backend::get_directory(x.id).await?;
        // setting the file
        //file_tree.set(FileTree { files: directory });
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
