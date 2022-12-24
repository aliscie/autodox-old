use std::collections::BTreeMap;
use surrealdb::sql::Thing;

use shared::{id::Id, schema::FileMode};
use uuid::Uuid;

use shared::{
    schema::{FileDirectory, FileNode, FileNodeCreate, FileNodeDelete},
    traits::Entity,
};
use tauri::State;

use crate::prelude::*;
use crate::utils::map;
use crate::Context;

/// TODO: wrap all the functions around transactions!
#[tauri::command]
pub async fn create_directory(data: FileDirectory, ctx: State<'_, Context>) -> Result<String> {
    let store = ctx.get_store();
    for (id, i) in &data.files.vertices {
        let children = data.files.adjacency.get(id).unwrap().clone();
        let file_create = FileNodeCreate {
            id: *id,
            name: i.name.clone(),
            children: Some(children),
            // mode: FileMode::Public,
            // these two doesn't matter using any value
            directory_id: Uuid::new_v4().into(),
            parent_id: Uuid::new_v4().into(),
        };
        let x = store.exec_create(file_create).await;
        println!("{:?}", x);
    }
    store.exec_create(data).await
}

#[tauri::command]
pub async fn create_file(data: FileNodeCreate, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let id = data.id;
    let parent_id = data.parent_id;
    let directory_id = data.directory_id;
    // println!("create_file data is : {:?}", data);
    store.exec_create(data).await?;
    // cannot use store.exec_update here due to pushing value to array
    let sql = "update $tb set children += $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((FileNode::table_name(), parent_id.to_string()).into()),
        "va".into() => Value::Thing((FileNode::table_name(), id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    // adding file to the vertices in file_directory
    let sql = "update $tb set files.vertices += $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((FileDirectory::table_name(), directory_id.to_string()).into()),
        "va".into() => Value::Thing((FileNode::table_name(), id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_directories(ctx: State<'_, Context>) -> Result<Vec<FileDirectory>> {
    let store = ctx.get_store();
    let res: Vec<FileDirectory> = store
        .exec_get::<FileDirectory>(None, Some("files.vertices.*.*"))
        .await?
        .into_iter()
        .map(|f| FileDirectory::try_from(f))
        .filter_map(|f| {
            println!("{:?}", f);
            f.ok()
        })
        .collect();
    println!("{:?}", res);
    Ok(res)
}

#[tauri::command]
pub async fn get_directory(id: Id, ctx: State<'_, Context>) -> Result<FileDirectory> {
    let store = ctx.get_store();
    let res = store
        .exec_get::<FileDirectory>(Some(id.to_string()), Some("files.vertices.*.*"))
        .await?
        .remove(0);
    Ok(res.try_into()?)
}

/// returns the ids of the deleted items
#[tauri::command]
pub async fn delete_file(data: FileNodeDelete, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let mut stack = vec![data.id];
    let mut current_index = 0;
    while current_index < stack.len() {
        let children: Vec<Value> = store
            .exec_delete::<FileNode>(stack[current_index].to_string())
            .await?
            .remove("children")
            .ok_or(Error::XPropertyNotFound("children".into()))?
            .try_into()?;
        for i in children {
            match i {
                Value::Thing(x) => {
                    let id = Uuid::parse_str(x.id.to_raw().as_str());
                    if let Ok(id) = id {
                        stack.push(id.into());
                    }
                }
                _ => continue,
            }
        }
        current_index += 1;
    }
    // deleting node from the parent_element
    let sql = "update $tb set children -= $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((FileNode::table_name(), data.parent_id.to_string()).into()),
        "va".into() => Value::Thing((FileNode::table_name(), data.id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    // deleting all nodes from the element_tree.vertices column
    let sql = "update $tb set files.vertices -= $va";
    for i in &stack {
        let vars: BTreeMap<String, Value> = map![
            "tb".into() => Value::Thing((FileDirectory::table_name(), data.tree_id.to_string()).into()),
            "va".into() => Value::Thing((FileNode::table_name(), i.to_string()).into()),
        ];
        store
            .datastore
            .execute(&sql, &store.session, Some(vars), false)
            .await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn change_directory(
    child_id: Uuid,
    parent_id: Uuid,
    old_parent_id: Uuid,
    ctx: State<'_, Context>,
) -> Result<()> {
    let store = ctx.get_store();
    let sql = "UPDATE $tb SET children -= $val";
    let vars: BTreeMap<String, Value> = BTreeMap::from([
        (
            "tb".into(),
            Thing::from((FileNode::table_name(), old_parent_id.to_string())).into(),
        ),
        (
            "val".into(),
            Thing::from((FileNode::table_name(), child_id.to_string())).into(),
        ),
    ]);
    store
        .datastore
        .execute(sql, &store.session, Some(vars), false)
        .await?;
    let sql = "UPDATE $tb SET children += $val";
    let vars: BTreeMap<String, Value> = BTreeMap::from([
        (
            "tb".into(),
            Thing::from((FileNode::table_name(), parent_id.to_string())).into(),
        ),
        (
            "val".into(),
            Thing::from((FileNode::table_name(), child_id.to_string())).into(),
        ),
    ]);
    store
        .datastore
        .execute(sql, &store.session, Some(vars), false)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use shared::schema::{FileDirectory, FileNode};

    use crate::{Context, Store};

    async fn setup() -> Context {
        let store = Store::new()
            .await
            .expect("Cannot create connection to database!");
        Context::new(store)
    }

    #[tokio::test]
    async fn test_create_file() {
        let context = setup().await;
        //let data = FileDirectory::default();
        //let object : Object = data.try_into().unwrap();
        //println!("{:?}", object);
        let mut data = FileDirectory::default();
        let file = FileNode::default();
        //data.files
        //.push_children(data.files.root.unwrap(), file.id, file);
        let store = context.get_store();
        //for i in data.api.vertices.values().into_iter() {
        //store.exec_create(i.clone()).await.unwrap();
        //}
        //store.exec_create(data).await.unwrap();
        //println!("{:?}", id);
        //let id = Uuid::from_str("80cc41c9-6239-469f-a7da-37bc8b6e17e9").unwrap();
        //let data = store.get_all::<FileDirectory>().await;
        //println!("{:?}", data);
        let res = store.exec_get::<FileDirectory>(None, None).await;
        println!("{:?}", res);
    }
}
