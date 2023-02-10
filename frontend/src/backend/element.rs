use crate::backend::call_surreal;
use crate::utils::DeviceInfo;
use shared::id::Id;
use shared::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use yewdux::prelude::Dispatch;

use super::call_ic;

pub async fn update_element(data: EditorElementUpdate) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return call_surreal(
            "update_element".to_string(),
            Some(&serde_json::json!({ "data": data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn delete_element(data: EditorElementDelete) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return call_surreal(
            "delete_element".to_string(),
            Some(&serde_json::json!({ "data": data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_element(data: EditorElementCreate) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return call_surreal(
            "create_element".to_string(),
            Some(&serde_json::json!({ "data": data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn get_element_tree(id: &Id) -> Result<ElementTree, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return call_surreal(
            "get_element_tree".to_string(),
            Some(&serde_json::json!({ "id": id })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_element_tree(data: &ElementTree, file_id: Id) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let result = call_ic(
            "create_element_tree".to_string(),
            serde_json::json!({
                "data" : data,
                "fileId" : file_id
            })
            .to_string(),
        )
        .await;
        return Ok(());
    }
    if !info.get().is_web {
        return call_surreal(
            "create_element_tree".to_string(),
            Some(&serde_json::json!({ "data": data , "fileId" : file_id})),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}
