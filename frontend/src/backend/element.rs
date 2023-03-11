use crate::backend::call_surreal;
use crate::utils::DeviceInfo;
use shared::id::Id;
use shared::schema::{EditorElementCreate, EditorElementDelete, EditorElementUpdate, ElementTree};
use yewdux::prelude::Dispatch;
use std::collections::HashMap;
use js_sys::Math::log;
use shared::log;
use crate::backend;
use super::call_ic;

pub async fn update_element(data: EditorElementUpdate) -> Result<(), String> {
    let info = Dispatch::<DeviceInfo>::new();
    // Note in the  info.get().is_web we don't do CRUD elements instead we do `grou_update`
    // if info.get().is_web || info.get().is_online {
    //     unimplemented!();
    // }
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
    // if info.get().is_web || info.get().is_online {
    //     unimplemented!();
    // }
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
    // if info.get().is_web || info.get().is_online {
    //     unimplemented!();
    // }
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
    // if info.get().is_web || info.get().is_online {
    //     unimplemented!();
    // }
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
                "file_id" : file_id
            })
                .to_string(),
        )
            .await;
        return Ok(());
    }
    if !info.get().is_web {
        return call_surreal(
            "create_element_tree".to_string(),
            Some(&serde_json::json!({ "data": data , "file_id" : file_id})),
        )
            .await;
    } else {
        // user is offline throw a error
        return Err("user is offline".to_string());
    }
}

pub async fn get_element_trees() -> Result<HashMap<Id, ElementTree>, String> {
    let info = Dispatch::<DeviceInfo>::new();
    if info.get().is_web || info.get().is_online {
        let response = backend::call_ic_np("get_element_trees".to_string()).await;
        let element_trees: Result<Option<Result<HashMap<Id, ElementTree>, String>>, _> = serde_wasm_bindgen::from_value(response);
        log!(&element_trees);
        // TODO
        //     type:"&core::result::Result<core::option::Option<std::collections::hash::map::HashMap<shared::data_fields::id::Id, shared::schema::element::ElementTree>>, serde_wasm_bindgen::error::Error>"
        //                 Err(
        //         Error(
        //             JsValue(Error: UUID parsing failed: invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `O` at 1
        //             Error: UUID parsing failed: invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `O` at 1
        //                 at imports.wbg.__wbindgen_error_new (http://localhost:5173/pkg/frontend.js:567:21)

        // if let Ok(Some(element_trees)) = element_trees {
        //     for (id, element_tree) in element_trees.iter() {
        //         log!(id);
        //         log!(element_tree);
        //     }
        // }


        return Err("not implemented".to_string());
    }
    return Err("user is offline".to_string());
}