use std::collections::{HashMap, HashSet, VecDeque};

use sea_orm::{ActiveValue, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, prelude::*, QuerySelect, Statement, TransactionTrait};

use shared::Tree;
use tauri::State;
use uuid::Uuid;

use crate::entity::file_adjacency::{self, Entity as FileAdjacency};
use crate::entity::file_node::{self, Entity as FileNode};
use crate::entity::file_tree::{self, Entity as FileTree};
use crate::utils::UuidSet;

async fn create_txn(db: State<'_, DatabaseConnection>) -> Result<DatabaseTransaction, std::string::String> {
    let db = db.inner();
    db.begin().await.map_err(|x| x.to_string())
}

pub async fn create_file_object(db: State<'_, DatabaseConnection>, new_obj: file_node::ActiveModel) -> Result<file_node::Model, String> {
    let txn = create_txn(db).await.unwrap();
    let node = FileNode::insert(new_obj)
        .exec_with_returning(&txn)
        .await
        .map_err(|x| x.to_string());
    txn.commit().await.map_err(|x| x.to_string());
    return node;
}


pub async fn create_tree_object(db: State<'_, DatabaseConnection>, new_obj: file_tree::ActiveModel) -> Result<file_tree::Model, String> {
    let txn = create_txn(db).await.unwrap();
    let tree = FileTree::insert(new_obj)
        .exec_with_returning(&txn)
        .await
        .map_err(|x| x.to_string());
    txn.commit().await.map_err(|x| x.to_string());
    return tree;
}