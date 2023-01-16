use shared::schema::FileDirectory;
use yewdux::prelude::Dispatch;

pub async fn init_files() -> Result<(), String> {
    let mut directories = crate::backend::get_directories()
        .await
        .map_err(|e| e)
        .unwrap_or_default();
    let file_tree = Dispatch::<FileDirectory>::new();
    match directories {
        Some(x) => {
            file_tree.set(x);
        }
        None => {
            let file_directory = FileDirectory::default();
            let x = crate::backend::create_directory(&file_directory).await;
            file_tree.set(file_directory);
        }
    }
    Ok(())
}
