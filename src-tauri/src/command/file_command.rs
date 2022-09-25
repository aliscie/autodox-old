use std::collections::{HashMap, HashSet};

use crate::entity::file_adjacency::{self, Entity as FileAdjacency};
use crate::entity::file_node::{self, Entity as FileNode};
use crate::entity::file_tree::{self, Entity as FileTree};
use crate::utils::UuidSet;
use sea_orm::{prelude::*, ActiveValue, DbBackend, QuerySelect, Statement, TransactionTrait};
use shared::Tree;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn get_directory(
    id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> Result<Tree<Uuid, file_node::Model>, String> {
    let x = FileTree::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| "Db Error".to_string())?;
    if x.is_none() {
        return Err("Not found!".to_string());
    }
    let x = x.unwrap();
    // rust iterator magic haha
    let adjacency = FileAdjacency::find()
        .filter(file_adjacency::Column::TreeId.eq(id))
        .select_only()
        .column(file_adjacency::Column::ParentId)
        .column(file_adjacency::Column::ChildId)
        .all(db.inner())
        .await
        .map_err(|x| x.to_string())?
        .into_iter()
        .map(|x| (x.parent_id, x.child_id.into()))
        .collect::<HashMap<Uuid, HashSet<Uuid>>>();
    // TODO : Finish this query
    let nodes: Vec<file_node::Model> = FileNode::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT "file_node"."id" , "file_node"."name", "file_node"."element_tree_id" FROM "file_node", "file_adjacency" 
                WHERE "file_adjacency"."tree_id" = $1 AND "file_node".id in (SELECT // read from
                // file_adjacency here and do some magic to convert json back into postgres array )
            "#,
            vec![id.into()],
        ))
        .all(db.inner())
        .await
        .map_err(|x| x.to_string())?;
    let vertices = nodes
        .into_iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<Uuid, file_node::Model>>();
    let tree = Tree {
        id,
        adjacency,
        vertices,
        root: x.root,
    };
    Ok(tree)
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
    let db = db.inner();
    let txn = db.begin().await.map_err(|x| x.to_string())?;
    let root = FileNode::insert(file_node::ActiveModel {
        name: ActiveValue::Set("root".to_string()),
        element_tree_id: ActiveValue::NotSet,
        ..Default::default()
    })
    .exec_with_returning(&txn)
    .await
    .map_err(|_| "Db Error".to_string())?;
    let x = FileTree::insert(file_tree::ActiveModel {
        name: ActiveValue::Set(name),
        root: ActiveValue::Set(Some(root.id)),
        ..Default::default()
    })
    .exec_with_returning(&txn)
    .await
    .map_err(|_| "Db Error".to_string())?;
    txn.commit().await.map_err(|x| x.to_string())?;
    return Ok(x);
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
        adj.child_id.0.insert(file.id);
        let m: file_adjacency::ActiveModel = adj.into();
        m.update(&txn).await.map_err(|x| x.to_string())?;
    } else {
        let m = file_adjacency::ActiveModel {
            tree_id: ActiveValue::Set(tree_id),
            parent_id: ActiveValue::Set(parent_id),
            child_id: ActiveValue::Set(UuidSet(HashSet::from([file.id]))),
        };
        FileAdjacency::insert(m)
            .exec(&txn)
            .await
            .map_err(|x| x.to_string())?;
    }

    txn.commit().await.map_err(|x| x.to_string())?;
    Ok(file)
}
