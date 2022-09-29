use crate::utils::DeviceInfo;
use crate::utils::FileDirectory;
use crate::utils::FileNode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use shared::{invoke_async, Tree};
use uuid::Uuid;
use yewdux::prelude::use_store;

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
    let (info, _) = use_store::<DeviceInfo>();
    if info.web || info.online {
        // TODO : add IC backend call here
        unimplemented!();
    }
    if !info.web {
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
    let (info, _) = use_store::<DeviceInfo>();
    if info.web || info.online {
        // TODO : add IC backend call here
        unimplemented!();
    }
    if !info.web {
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
    let (info, _) = use_store::<DeviceInfo>();
    if info.web || info.online {
        unimplemented!();
    }
    if !info.web {
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

pub async fn create_file(tree_id: Uuid, parent_id: Uuid, name: String) -> Result<FileNode, String> {
    let (info, _) = use_store::<DeviceInfo>();
    if info.web || info.online {
        unimplemented!();
    }
    if !info.web {
        return call_postgres(
            "create_file".to_string(),
            Some(&serde_json::json!({
                "tree_id" : tree_id,
                "parent_id" : parent_id,
                "name" : name
            })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn delete_file(tree_id: Uuid, file_id: Uuid) -> Result<(), String> {
    let (info, _) = use_store::<DeviceInfo>();
    if info.web || info.online {
        unimplemented!();
    }
    if !info.web {
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
