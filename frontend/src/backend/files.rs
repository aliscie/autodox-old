use std::str::FromStr;

use crate::backend;
use crate::utils::DeviceInfo;
use futures::future::err;
use shared::schema::ElementTree;
use shared::{
    id::Id,
    log,
    schema::{
        FileDirectory, FileNode, FileNodeCreate, FileNodeDelete, FileNodeMove, FileNodeUpdate,
    },
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};
use yewdux::prelude::{use_store, Dispatch};

pub async fn rename_file(id: Id, new_name: String) -> Result<(), String> {
    let data = FileNodeUpdate {
        id: id.into(),
        name: Some(new_name),
        children: None,
        parent_id: None,
        element_tree: None,
    };
    let curr = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .get_element_by_id(&id.to_string())
        .unwrap();
    let _ = curr.class_list().toggle("loader");
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let _ = backend::call_ic(
            "rename_file".to_string(),
            serde_json::json!(data).to_string(),
        )
            .await;
        let _ = curr.class_list().toggle("loader");
        return Ok(());
    } else {
        log!("rename on desktop");
        let _ = curr.class_list().toggle("loader");
        return Err("user is offline".to_string());
    }
}

pub async fn create_file(tree_id: Id, parent_id: Id, name: String, id: Id) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    let data = FileNodeCreate {
        directory_id: tree_id.into(),
        parent_id: parent_id.into(),
        name,
        id: id.into(),
        children: None,
    };
    // log!(&data);
    if info.get().is_web || info.get().is_online {
        let file_json = serde_json::json!(data);
        log!(&file_json);
        let res = backend::call_ic("create_file".to_string(), file_json.to_string()).await;
        log!(&res);
        return Ok(());
    } else if !info.get().is_web {
        log!("Desktop");
        let new_file = serde_json::json!({ "data": data });
        return backend::call_surreal("create_file".to_string(), Some(&new_file)).await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn update_file(file_node: FileNode) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let file_json = serde_json::json!(file_node);
        let curr = window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .get_element_by_id(&file_node.id.to_string())
            .unwrap();
        let _ = curr.class_list().add_1("loader");
        let res = backend::call_ic("update_file".to_string(), file_json.to_string()).await;
        let _ = curr.class_list().remove_1("loader");
        log!(&res);
        return Ok(());
    } else if !info.get().is_web {
        unimplemented!();
    } else {
        return Err("user is offline".to_string());
    }
}

pub async fn delete_file(data: FileNodeDelete) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let data_json = serde_json::json!(data).to_string();
        let curr = window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .get_element_by_id(&data.id.to_string())
            .unwrap();
        let _ = curr.class_list().toggle("loader");
        log!(&data_json);
        let res = backend::call_ic("delete_file".to_string(), data_json).await;
        let _ = curr.class_list().toggle("loader");
        log!(&res);
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
        spawn_local(async move {
            let response = backend::call_ic_np("create_directory".to_string()).await;
            log!(response);
        })
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

pub async fn get_directory(id: Id, auther_id: Id) -> Result<FileDirectory, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        //
        let input_data = serde_json::json!((auther_id, id)).to_string();
        let file_node_jv = backend::call_ic("get_file".to_string(), input_data.to_string()).await;
        // let file_node_res = serde_wasm_bindgen::from_value::<Option<FileNode>>(file_node_jv);
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

// pub async fn get_file(id: Id) -> Result<Option<ElementTree>, String> {
//
//     // TODO
//     //  if File exists in Yewdux then return it
//     //  else fetch it from IC backend and return it or Err("No such file in the IC nor in the Yewdux".to_string());
//
//     let info = Dispatch::<DeviceInfo>::new();
//     // let file_node = Dispatch::<FileDirectory>::new();
//     // let (files, _) = use_store::<FileDirectory>();
//     // let file = files.clone().get().files.vertices.get(&id);
//     // if let Some(file) = file {
//     //     return Ok(Some(file.clone()));
//     // }
//
//     if info.get().is_web {
//         if info.get().is_online {
//             return Err("web and online".to_string());
//         } else {
//             return Err("web and not online".to_string());
//         }
//     } else if info.get().is_online {
//         return Err("Desktop and online".to_string());
//     } else {
//         return Err("Desktop and not online".to_string());
//     }
// }

pub async fn get_directories() -> Result<Option<FileDirectory>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        // let response = backend::call_ic_np("get_directories".to_string()).await;
        let file_tree_component = window().unwrap_throw().document().unwrap_throw().query_selector("#aside-content").unwrap().unwrap();
        file_tree_component.class_list().add_1("loader");
        let response = backend::get_directories_ic().await;
        let file_tree: Result<Option<FileDirectory>, _> = serde_wasm_bindgen::from_value(response);
        file_tree_component.class_list().remove_1("loader");
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

pub fn change_directory(parent_id: String, child_id: String, old_parent_id: String) {
    spawn_local(async move {
        let x = self::local_change_directory(parent_id, child_id, old_parent_id).await;
        console::log_1(&format!("change_directory returned : {:?}", x).into())
    });
}

async fn local_change_directory(
    parent_id: String,
    child_id: String,
    old_parent_id: String,
) -> Result<(), String> {
    let file_node_move = FileNodeMove {
        id: Id::from_str(&child_id).unwrap(),
        old_parent_id: Id::from_str(&old_parent_id).unwrap(),
        new_parent_id: Id::from_str(&parent_id).unwrap(),
    };
    let info = Dispatch::<DeviceInfo>::new();
    let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
    if info.get().is_web || info.get().is_online {
        let json_file_node_move = serde_json::json!(file_node_move).to_string();
        let res = backend::call_ic("change_directory".to_string(), json_file_node_move).await;
        return Ok(());
    }
    if !info.get().is_web {
        return crate::backend::call_surreal(
            "change_directory".to_string(),
            Some(&serde_json::json!({ "childId": child_id , "parentId" : parent_id, "oldParentId" : old_parent_id})),
        )
            .await
            .map(|e| {
                _dispatch_file_directory.reduce_mut(|f| {
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
