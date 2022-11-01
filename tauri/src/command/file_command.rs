use crate::prelude::*;
use crate::utils::map;
use crate::Context;
use shared::{
    schema::{FileDirectory, FileNodeCreate},
    traits::Entity,
};
use std::collections::BTreeMap;
use surrealdb::sql::*;
use tauri::State;
use uuid::Uuid;

/// TODO: wrap all the functions around transactions!
#[tauri::command]
pub async fn create_directory(data: FileDirectory, ctx: State<'_, Context>) -> Result<String> {
    let store = ctx.get_store();
    for (_, i) in &data.files.vertices {
        store.exec_create(i.clone()).await?;
    }
    store.exec_create(data).await
}

#[tauri::command]
pub async fn create_file(data: FileNodeCreate, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let id = data.id;
    let directory_id = data.directory_id;
    let parent_id = data.parent_id;
    store.exec_create(data).await?;
    let sql = format!("update $tb set files.adjacency.`{:?}` += $va", parent_id);
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((FileDirectory::table_name(), directory_id.to_string()).into()),
        "va".into() => format!("{:?}", id).into(),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    let sql = format!("update $tb set files.adjacency.`{:?}` = []", id);
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((FileDirectory::table_name(), directory_id.to_string()).into()),
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
pub async fn get_directory(id: Uuid, ctx: State<'_, Context>) -> Result<FileDirectory> {
    let store = ctx.get_store();
    let res = store
        .exec_get::<FileDirectory>(Some(id.to_string()), Some("files.vertices.*.*"))
        .await?
        .remove(0);
    Ok(res.try_into()?)
}

#[cfg(test)]
mod tests {
    use crate::{Context, Store};
    use shared::schema::{FileDirectory, FileNode};
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
        data.files
            .push_children(data.files.root.unwrap(), file.id, file);
        let store = context.get_store();
        //for i in data.files.vertices.values().into_iter() {
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
