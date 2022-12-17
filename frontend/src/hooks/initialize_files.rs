use yewdux::prelude::Dispatch;
use shared::schema::FileDirectory;

pub async fn init_files() -> Result<(), String> {
    let mut directories = crate::backend::get_directories().await.map_err(|e| {
        e
    }).unwrap_or_default();
    let file_tree = Dispatch::<FileDirectory>::new();
    if directories.len() == 0 {
        let file_directory = FileDirectory::default();
        let x = crate::backend::create_directory(&file_directory).await;
        file_tree.set(file_directory);
    } else {
        let x = directories.remove(0);
        file_tree.set(x);
    }
    Ok(())
}
