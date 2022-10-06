use sea_orm::{prelude::*, ActiveValue, DbBackend, QuerySelect, Statement, TransactionTrait, DatabaseTransaction};
use shared::Tree;
use std::collections::{HashMap, HashSet};
use tauri::State;
use uuid::Uuid;

use crate::entity::file_adjacency::{self, Entity as FileAdjacency};
use crate::entity::file_node::{self, Entity as FileNode};
use crate::entity::file_tree::{self, Entity as FileTree};
use crate::utils::UuidSet;

pub struct HandleData {}

impl HandleData {
    async fn create_txn(self, db: State<'_, DatabaseConnection>) -> Result<DatabaseTransaction, std::string::String> {
        let db = db.inner();
        db.begin().await.map_err(|x| x.to_string())
    }

    async fn create_file(self, db: State<'_, DatabaseConnection>, new_obj: file_node::ActiveModel) -> Result<file_node::Model, String> {
        let txn = self.create_txn(db).await.unwrap();
        let node = FileNode::insert(new_obj)
            .exec_with_returning(&txn)
            .await
            .map_err(|x| x.to_string());
        txn.commit().await.map_err(|x| x.to_string());
        return node;
    }

    async fn create_tree(self, db: State<'_, DatabaseConnection>, new_obj: file_tree::ActiveModel) -> Result<file_tree::Model, String> {
        let txn = self.create_txn(db).await.unwrap();
        let tree = FileTree::insert(new_obj)
            .exec_with_returning(&txn)
            .await
            .map_err(|x| x.to_string());
        txn.commit().await.map_err(|x| x.to_string());
        return tree;
    }

    async fn create_directory(
        self,
        name: String,
        db: State<'_, DatabaseConnection>,
    ) -> Result<file_tree::Model, String> {
        let root = &self.create_file(db, file_node::ActiveModel {
            name: ActiveValue::Set("root".to_string()),
            element_tree_id: ActiveValue::NotSet,
            id: ActiveValue::Set(Uuid::new_v4()),
        });

        let x = &self.create_tree(db, file_tree::ActiveModel {
            name: ActiveValue::Set(name),
            root: ActiveValue::Set(Some(root.id)),
            id: ActiveValue::Set(Uuid::new_v4()),
        });
        return Ok(x);
    }


    async fn create_file_object(
        tree_id: Uuid,
        parent_id: Uuid,
        name: String,
        db: State<'_, DatabaseConnection>,
    ) -> Result<file_node::Model, String> {
        let file = &self.create_file(db, file_node::ActiveModel {
            name: ActiveValue::Set("root".to_string()),
            element_tree_id: ActiveValue::NotSet,
            id: ActiveValue::Set(Uuid::new_v4()),
        });

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
}