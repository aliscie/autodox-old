use crate::utils::DeviceInfo;
use crate::utils::FileDirectory;
use crate::utils::FileNode;
use crate::utils::FileTree;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use shared::{invoke_async, Tree};
use uuid::Uuid;
use yewdux::prelude::Dispatch;

// TODO : we can make both the calls to postgres and IC backend parrallel

// generic magic haha
pub async fn call_postgres<T, U>(command: String, args: Option<&U>) -> Result<T, String>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let x = invoke_async::<U>(command, args)
        .await
        .map_err(|e| format!("{:?}", e))?;
    from_value(x).map_err(|e| e.to_string())
}

pub async fn get_directories() -> Result<Vec<FileDirectory>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        let x =
            call_postgres::<Vec<FileDirectory>, String>("get_directories".to_string(), None).await;
        return x;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

/// returns all the files in a directory
pub async fn get_directory(id: Uuid) -> Result<Tree<Uuid, FileNode>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return call_postgres(
            "get_directory".to_string(),
            Some(&serde_json::json!({ "id": id })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_directory(name: String) -> Result<FileDirectory, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return call_postgres(
            "create_directory".to_string(),
            Some(&serde_json::json!({ "name": name })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn change_directory(parent_id: String, child_id: String) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let file_dispatch = Dispatch::<FileTree>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return call_postgres(
            "change_directory".to_string(),
            Some(&serde_json::json!({ "childId": child_id , "parentId" : parent_id})),
        )
        .await
        .map(|e| {
            file_dispatch.reduce_mut(|f| {
                for i in f.files.adjacency.values_mut() {
                    i.remove(&Uuid::parse_str(&child_id).unwrap());
                }
                f.files.push_edge(
                    Uuid::parse_str(&parent_id).unwrap(),
                    Uuid::parse_str(&child_id).unwrap(),
                );
            });
            e
        });
    }
    return Err("user is offline".to_string());
}

pub async fn create_file(tree_id: Uuid, parent_id: Uuid, name: String) -> Result<FileNode, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        let x = serde_json::json!({
            "treeId" : tree_id,
            "parentId" : parent_id,
            "name" : name
        });
        gloo::console::log!(format!("{:?}", x));
        return call_postgres("create_file".to_string(), Some(&x)).await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn delete_file(tree_id: Uuid, file_id: Uuid) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return call_postgres(
            "delete_file".to_string(),
            Some(&serde_json::json!({"tree_id" : tree_id, "file_id" : file_id})),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn on_startup() -> Result<(), String> {
    let directoies = get_directories().await?;
    let file_tree = Dispatch::<FileTree>::new();
    if directoies.len() == 0 {
        // create new directory tree
        let x = create_directory("default".to_string()).await?;
        let directory = get_directory(x.id).await?;
        // setting the file
        file_tree.set(FileTree { files: directory });
    } else {
        let x = directoies.get(0).unwrap();
        let directory = get_directory(x.id).await?;
        // setting the file
        file_tree.set(FileTree { files: directory });
    }
    Ok(())
}
