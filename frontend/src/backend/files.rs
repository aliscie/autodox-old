use crate::utils::DeviceInfo;
use shared::schema::{FileDirectory, FileNode};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::Dispatch;

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
        return crate::backend::call_surreal("create_file".to_string(), Some(&x)).await;
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
        return crate::backend::call_surreal(
            "delete_file".to_string(),
            Some(&serde_json::json!({"tree_id" : tree_id, "file_id" : file_id})),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn create_directory(_name: String) -> Result<FileDirectory, String> {
    let info = Dispatch::<DeviceInfo>::new();
    let data = FileDirectory::default();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
            "create_directory".to_string(),
            Some(&serde_json::json!({ "data" : data})),
        )
        .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn get_directory(id: Uuid) -> Result<FileDirectory, String> {
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

pub fn change_directory(parent_id: String, child_id: String)
// -> Result<(), String>
{
    // let mut response: Option<Result<(), String>> = None;
    spawn_local(async move {
        self::local_change_directory(parent_id, child_id).await;
    });
    // return response.unwrap();
}

async fn local_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let file_dispatch = Dispatch::<FileDirectory>::new();
    if info.get().web || info.get().online {
        unimplemented!();
    }
    if !info.get().web {
        return crate::backend::call_surreal(
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

//  async fn cloud_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
//     ....
// }

//  async fn synced_change_directory(parent_id: String, child_id: String) -> Result<(), String> {
//     backends::local_change_directory(parent_id,child_id);
//     backends::cloud_change_directory(parent_id,child_id);
// }

//
//
// fn add_file() {
//     let mut new_file: Option<None> = Some(None);
//
//     if IS_WEB {
//         params = [("method", "add_file"), ("name", "untitled")];
//         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
//         new_file = res.file
//     } else {
//         let file = crate::backend::files::create_file(
//             state.files.id,
//             state.files.root.unwrap(),
//             "untitled".to_string(),
//         )
//             .await;
//         new_file = file
//     }
//
//     if let Ok(f) = file {
//         state.files.push_children(state.files.root.unwrap(), f.id, f);
//     }
//
// }
//
// fn get_files() -> Vec<FileTree> {
//     let files = [];
//     if IS_LOGEDIN {
//         params = [("method", "files")];
//         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
//         files.extends(res.files)
//     }
//     if IS_WEB == false {
//         files.extends(get_directory());
//     }
//     return files;
// }
//
// fn delete_file(id: Uuid, file_location: FileLocation) {
//     if file_location == FileLocation.LOCAL {
//         tauri::commands: delete_file(id);
//     }
//     if file_location == FileLocation.CLOUD {
//         params = [("method", "delete_file"), ("id", id)];
//         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
//     }
//
//     if file_location == FileLocation.SYNCED {
//         tauri::commands: delete_file(id);
//         params = [("method", "delete_file"), ("id", id)];
//         let res = reqwest::post("host.io?canister_id=aaa-aa").from(params);
//     }
// }
