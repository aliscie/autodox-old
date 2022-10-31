use crate::prelude::*;
use std::collections::BTreeMap;
use crate::Context;
use shared::schema::{FileDirectory, FileNodeCreate};
use tauri::State;
use uuid::Uuid;
use surrealdb::sql::{Value, Thing};
use crate::utils::map;

#[tauri::command]
pub async fn create_directory(data: FileDirectory, ctx: State<'_, Context>) -> Result<String> {
    let store = ctx.get_store();
    for (_, i) in &data.files.vertices {
        store.exec_create(i.clone()).await;
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
    let sql = "update $tb set files.adjacency += $ad , files.vertices += $ve";
    Ok(())
}

#[tauri::command]
pub async fn get_directories(ctx: State<'_, Context>) -> Result<Vec<FileDirectory>> {
    let store = ctx.get_store();
    let mut result = Vec::new();
    for i in store.get_all::<FileDirectory>().await {
        result.push(i.try_into()?);
    }
    Ok(result)
}

pub async fn get_directory(id: Uuid, ctx: State<'_, Context>) -> Result<FileDirectory> {
    let store = ctx.get_store();
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{Context, Store};
    use shared::schema::FileDirectory;
    use std::str::FromStr;
    use surrealdb::sql::Object;
    use uuid::Uuid;
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
        //let data = FileDirectory::default();
        let store = context.get_store();
        //let id = store.exec_create(data).await.unwrap();
        //println!("{:?}", id);
        //let id = Uuid::from_str("80cc41c9-6239-469f-a7da-37bc8b6e17e9").unwrap();
        let data = store.get_all::<FileDirectory>().await;
        println!("{:?}", data);
    }
}
