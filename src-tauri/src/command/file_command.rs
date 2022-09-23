//use crate::entity::file_tree::{Entity as FileTree, self};
//use sea_orm::prelude::*;
//use tauri::State;
//use uuid::Uuid;

//#[tauri::command]
//pub async fn get_directory(
    //id: Uuid,
    //db: State<'_, DatabaseConnection>,
//) -> Result<Option<file_tree::Model>, sea_orm::DbErr> {
    //FileTree::find_by_id(id).one(db.inner()).await
//}
