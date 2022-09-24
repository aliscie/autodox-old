use crate::entity::file_adjacency::{self, Entity as FileAdjacency, UuidVec};
use crate::entity::file_node::{self, Entity as FileNode};
use crate::entity::file_tree::{self, Entity as FileTree};
use sea_orm::{prelude::*, ActiveValue, TransactionTrait};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn get_directory(
    id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> Result<file_tree::Model, String> {
    let x = FileTree::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| "Db Error".to_string())?;
    match x {
        Some(x) => Ok(x),
        None => Err("Not found!".to_string()),
    }
}

#[tauri::command]
pub async fn get_directories(
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<file_tree::Model>, String> {
    FileTree::find()
        .all(db.inner())
        .await
        .map_err(|_| "Db Error".to_string())
}

#[tauri::command]
pub async fn create_directory(
    name: String,
    db: State<'_, DatabaseConnection>,
) -> Result<file_tree::Model, String> {
    FileTree::insert(file_tree::ActiveModel {
        name: ActiveValue::Set(name),
        ..Default::default()
    })
    .exec_with_returning(db.inner())
    .await
    .map_err(|_| "Db Error".to_string())
}

#[tauri::command]
pub async fn create_file(
    tree_id: Uuid,
    parent_id: Uuid,
    name: String,
    db: State<'_, DatabaseConnection>,
) -> Result<file_node::Model, String> {
    let db = db.inner();
    let txn = db.begin().await.map_err(|x| x.to_string())?;
    let file = FileNode::insert(file_node::ActiveModel {
        name: ActiveValue::Set(name),
        ..Default::default()
    })
    .exec_with_returning(&txn)
    .await
    .map_err(|x| x.to_string())?;
    let mut adj = FileAdjacency::find_by_id((tree_id, parent_id))
        .one(&txn)
        .await
        .map_err(|x| x.to_string())?;
    if let Some(mut adj) = adj {
        adj.child_id.0.push(file.id);
        let m: file_adjacency::ActiveModel = adj.into();
        m.update(&txn).await.map_err(|x| x.to_string())?;
    } else {
        let m = file_adjacency::ActiveModel {
            tree_id: ActiveValue::Set(tree_id),
            parent_id: ActiveValue::Set(parent_id),
            child_id: ActiveValue::Set(UuidVec(vec![file.id])),
        };
        FileAdjacency::insert(m)
            .exec(&txn)
            .await
            .map_err(|x| x.to_string())?;
    }

    txn.commit().await.map_err(|x| x.to_string())?;
    Ok(file)
}

