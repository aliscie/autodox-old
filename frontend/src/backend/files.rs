use uuid::Uuid;
use shared::{log, schema::FileNodeDelete};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yewdux::prelude::Dispatch;

use crate::utils::DeviceInfo;
use shared::{
    id::Id,
    schema::{FileDirectory, FileNodeCreate},
};

pub async fn create_file(
    tree_id: Id,
    parent_id: Id,
    name: String,
    id: Id,
) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let data = FileNodeCreate {
        directory_id: tree_id.into(),
        parent_id: parent_id.into(),
        name,
        id: id.into(),
        children: None,
    };
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        let new_file = serde_json::json!({ "data": data });
        return crate::backend::call_surreal("create_file".to_string(), Some(&new_file)).await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn delete_file(data : FileNodeDelete) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
            "delete_file".to_string(),
            Some(&serde_json::json!({ "data" : data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_directory(data: &FileDirectory) -> Result<String, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
            "create_directory".to_string(),
            Some(&serde_json::json!({ "data": data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn get_directory(id: Id) -> Result<FileDirectory, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
            "get_directory".to_string(),
            Some(&serde_json::json!({ "id": id })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn get_directories() -> Result<Vec<FileDirectory>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        let x = crate::backend::call_surreal::<Vec<FileDirectory>, String>(
            "get_directories".to_string(),
            None,
        )
        .await;
        log!(&x);
        return x;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

enum FileLocation {
    LOCAL,
    CLOUD,
    SYNCED,
}

pub fn change_directory(parent_id: String, child_id: String, old_parent_id: String)
// -> Result<(), String>
{
    // let mut response: Option<Result<(), String>> = None;
    spawn_local(async move {
        let x = self::local_change_directory(parent_id, child_id, old_parent_id).await;
        console::log_1(&format!("change_directory returned : {:?}", x).into())
    });
    // return response.unwrap();
}

async fn local_change_directory(
    parent_id: String,
    child_id: String,
    old_parent_id: String,
) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let file_dispatch = Dispatch::<FileDirectory>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
            "change_directory".to_string(),
            Some(&serde_json::json!({ "childId": child_id , "parentId" : parent_id, "oldParentId" : old_parent_id})),
        )
            .await
            .map(|e| {
                file_dispatch.reduce_mut(|f| {
                    for i in f.files.adjacency.values_mut() {
                        i.remove(&Uuid::parse_str(&child_id).map(Id::from).unwrap());
                    }
                    f.files.push_edge(
                        Uuid::parse_str(&parent_id).map(Id::from).unwrap(),
                        Uuid::parse_str(&child_id).map(Id::from).unwrap(),
                    );
                });
                e
            });
    }
    return Err("user is offline".to_string());
}
