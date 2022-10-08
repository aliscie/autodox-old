pub mod file_command;
pub mod element;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use tauri::State;
mod window_commands;
// pub mod handle_data;
pub async fn create_txn(db: State<'_, DatabaseConnection>) -> Result<DatabaseTransaction, std::string::String> {
    let db = db.inner();
    db.begin().await.map_err(|x| x.to_string())
}
