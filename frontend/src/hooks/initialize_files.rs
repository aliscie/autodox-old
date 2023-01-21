use shared::schema::FileDirectory;
use shared::*;
use yewdux::prelude::Dispatch;

pub async fn init_files() -> Result<(), String> {
    let mut directories = crate::backend::get_directories()
        .await
        .map_err(|e| e)
        .unwrap_or_default();
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();

    match directories {
        Some(x) => {
            dispatch_file_directory.set(x);
        }
        None => {
            let file_directory = FileDirectory::default();
            let x = crate::backend::create_directory(&file_directory).await;
            dispatch_file_directory.set(file_directory);
        }
    }

    Ok(())
}
