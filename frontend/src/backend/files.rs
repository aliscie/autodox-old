use futures::future::err;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yewdux::prelude::Dispatch;

use shared::{
    id::Id,
    schema::{FileDirectory, FileNodeCreate},
};
use shared::{log, schema::FileNodeDelete};

use crate::backend;
use crate::utils::DeviceInfo;

pub async fn create_file(tree_id: Id, parent_id: Id, name: String, id: Id) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let data = FileNodeCreate {
        directory_id: tree_id.into(),
        parent_id: parent_id.into(),
        name,
        // using this for right now
        // mode: shared::schema::FileMode::Public,
        id: id.into(),
        children: None,
    };
    if info.get().is_web || info.get().is_online {
        // spawn_local(async move {
        //     log!("before create file");
        //     let res = backend::create_file_ic().await;
        //     log!(res);
        // });
    }
    if !info.get().is_web {
        log!("Desktop");
        let new_file = serde_json::json!({ "data": data });
        return crate::backend::call_surreal("create_file".to_string(), Some(&new_file)).await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn delete_file(data: FileNodeDelete) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return crate::backend::call_surreal(
            "delete_file".to_string(),
            Some(&serde_json::json!({ "data": data })),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_directory(data: &FileDirectory) -> Result<String, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let response = backend::create_directory_ic().await;
        log!(response);
    }
    if !info.get().is_web {
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
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
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

pub async fn get_directories() -> Result<Option<FileDirectory>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        // TODO backend::call_ic("create_directory"); make this dynamic
        log!("before get dirs");
        let response = backend::get_directories_ic().await;
        log!(&response);
        let file_tree: Result<Option<FileDirectory>, serde_wasm_bindgen::Error> =
            serde_wasm_bindgen::from_value(response);
        log!(&file_tree);
        return file_tree.map_err(|e| "serde error".to_string());
    }
    if !info.get().is_web {
        let x = crate::backend::call_surreal::<Option<FileDirectory>, String>(
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
    if info.get().is_web || info.get().is_online {
        unimplemented!();
    }
    if !info.get().is_web {
        return crate::backend::call_surreal(
            "change_directory".to_string(),
            Some(&serde_json::json!({ "childId": child_id , "parentId" : parent_id, "oldParentId" : old_parent_id})),
        )
            .await
            .map(|e| {
                file_dispatch.reduce_mut(|f| {
                    let child_id = Uuid::parse_str(&child_id).map(Id::from).unwrap();
                    for i in f.files.adjacency.values_mut() {
                        i.iter()
                            .position(|x| *x == child_id)
                            .map(|index| i.remove(index));
                    }
                    f.files.push_edge(
                        Uuid::parse_str(&parent_id).map(Id::from).unwrap(),
                        child_id,
                    );
                });
                e
            });
    }
    return Err("user is offline".to_string());
}
