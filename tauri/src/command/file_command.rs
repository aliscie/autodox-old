use std::collections::{HashMap, HashSet, VecDeque};

use crate::entity::file_adjacency::{self, Entity as FileAdjacency};
use crate::entity::file_node::{self, Entity as FileNode};
use crate::entity::file_tree::{self, Entity as FileTree};
use crate::utils::UuidSet;
use sea_orm::{
    prelude::*, ActiveValue, ConnectionTrait, DatabaseTransaction, DbBackend, QuerySelect,
    Statement, TransactionTrait,
};
use shared::Tree;
use tauri::State;
use uuid::Uuid;

#[allow(dead_code)]
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
    println!("{:?}", x);
    // rust iterator magic haha
    let adjacency = FileAdjacency::find()
        .filter(file_adjacency::Column::TreeId.eq(id))
        .all(db.inner())
        .await
        .map_err(|x| x.to_string())?
        .into_iter()
        .map(|x| (x.parent_id, x.child_id.into()))
        .collect::<HashMap<Uuid, HashSet<Uuid>>>();
    println!("{:?}", adjacency);
    // DO NOT TOUCH THIS QUERY TOOK ME 2 HOURS TO WRITE
    let nodes: Vec<file_node::Model> = FileNode::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                SELECT "file_node"."id" , "file_node"."name", "file_node"."element_tree_id" FROM "file_node" , "file_adjacency"
                WHERE (to_jsonb(ARRAY["file_node"."id"::text]) <@ "file_adjacency"."child_id"
                OR "file_node"."id" = "file_adjacency"."parent_id") 
                AND "file_adjacency"."tree_id" = $1
                GROUP BY "file_node".id
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

#[allow(dead_code)]
#[tauri::command]
pub async fn get_directories(
    db: State<'_, DatabaseConnection>,
) -> Result<Vec<file_tree::Model>, String> {
    FileTree::find()
        .all(db.inner())
        .await
        .map_err(|_| "Db Error".to_string())
}

#[allow(dead_code)]
#[tauri::command]
pub async fn create_directory(
    name: String,
    db: State<'_, DatabaseConnection>,
) -> Result<file_tree::Model, String> {
    let db = db.inner();
    let txn: DatabaseTransaction = db.begin().await.map_err(|x| x.to_string())?;
    let root = FileNode::insert(file_node::ActiveModel {
        name: ActiveValue::Set("root".to_string()),
        element_tree_id: ActiveValue::NotSet,
        id: ActiveValue::Set(Uuid::new_v4()),
    })
    .exec_with_returning(&txn)
    .await
    .map_err(|_| "Db Error".to_string())?;
    let new_obj: file_tree::ActiveModel = file_tree::ActiveModel {
        name: ActiveValue::Set(name),
        root: ActiveValue::Set(Some(root.id)),
        id: ActiveValue::Set(Uuid::new_v4()),
    };
    let x = FileTree::insert(new_obj)
        .exec_with_returning(&txn)
        .await
        .map_err(|e| e.to_string())?;
    FileAdjacency::insert(file_adjacency::ActiveModel {
        tree_id: ActiveValue::Set(x.id),
        parent_id: ActiveValue::Set(x.root.unwrap()),
        ..Default::default()
    })
    .exec(&txn)
    .await
    .map_err(|e| e.to_string())?;
    txn.commit().await.map_err(|x| x.to_string())?;
    return Ok(x);
}

#[allow(dead_code)]
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
        id: ActiveValue::Set(Uuid::new_v4()),
        ..Default::default()
    })
    .exec_with_returning(&txn)
    .await
    .map_err(|e| e.to_string())?;
    let adj = FileAdjacency::find_by_id((tree_id, parent_id))
        .one(&txn)
        .await
        .map_err(|x| x.to_string())?;
    if let Some(adj) = adj {
        // use this query
        // update file_adjacency set child_id = child_id || to_jsonb(ARRAY['b9a51dc9-7ed4-469e-a6da-af8608a6cfc3'::text])
        txn.query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"UPDATE file_adjacency SET child_id = child_id || to_jsonb(ARRAY[$1::text]) WHERE parent_id = $2"#,
            [file.id.into(), adj.parent_id.into()],
        ))
        .await
        .map_err(|x| x.to_string())?;
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
    // so on delete we can only delete and it will cascade to the real row!
    FileAdjacency::insert(file_adjacency::ActiveModel {
        tree_id: ActiveValue::Set(tree_id),
        parent_id: ActiveValue::Set(file.id),
        child_id: ActiveValue::Set(HashSet::new().into()),
    })
    .exec(&txn)
    .await
    .map_err(|x| x.to_string())?;
    txn.commit().await.map_err(|x| x.to_string())?;
    Ok(file)
}

#[allow(dead_code)]
#[tauri::command]
pub async fn delete_file(
    tree_id: Uuid,
    file_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let txn = db.inner().begin().await.map_err(|x| x.to_string())?;
    let mut stack: VecDeque<Uuid> = VecDeque::from([file_id]);
    // SUGGESTION : replace this with a recursive postogres call!
    while stack.len() > 0 {
        let id = stack.pop_front().unwrap();
        let x = FileAdjacency::find_by_id((tree_id, id))
            .select_only()
            .column(file_adjacency::Column::ChildId)
            .one(&txn)
            .await
            .map_err(|x| x.to_string())?
            .unwrap();
        for i in x.child_id.iter() {
            stack.push_front(*i);
        }
        x.delete(&txn).await.map_err(|x| x.to_string())?;
    }
    txn.commit().await.map_err(|x| x.to_string())?;
    Ok(())
}

#[allow(dead_code)]
#[tauri::command]
pub async fn change_directory(
    parent_id: Uuid,
    child_id: Uuid,
    db: State<'_, DatabaseConnection>,
) -> Result<(), String> {
    let txn = db.inner().begin().await.map_err(|x| x.to_string())?;
    txn.query_one(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"UPDATE file_adjacency SET child_id = child_id - ARRAY[$1::text]"#,
        [child_id.into()],
    ))
    .await
    .map_err(|x| x.to_string())?;
    txn.query_one(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"UPDATE file_adjacency SET child_id = child_id || to_jsonb(ARRAY[$1::text]) WHERE
        parent_id = $2"#,
        [child_id.into(), parent_id.into()],
    ))
    .await
    .map_err(|x| x.to_string())?;
    txn.commit().await.map_err(|x| x.to_string())?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn create_directory() {
        //let conn = crate::connect_database(String::from(crate::POSTGRES_URL)).await;
        //TODO : Find way to test command how to convert DatabaseConnection into
        //tauri::State<DatabaseConnection>
    }
}
