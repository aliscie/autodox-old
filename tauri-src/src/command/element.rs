use crate::context::Context;
use crate::prelude::*;
use crate::utils::*;
use shared::schema::*;
use shared::traits::Entity;
use std::collections::BTreeMap;
use surrealdb::sql::Value;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_element_tree(
    file_id: Uuid,
    data: ElementTree,
    ctx: State<'_, Context>,
) -> Result<()> {
    let store = ctx.get_store();
    for (id, i) in &data.elements.vertices {
        let children = data.elements.adjacency.get(id).unwrap().clone();
        let element_create = EditorElementCreate {
            id: *id,
            text: i.text.clone(),
            children: Some(children),
            attrs: i.attrs.clone(),
            // these doesn't matter we are throwing
            parent_id: Uuid::new_v4().into(),
            tree_id: Uuid::new_v4().into(),
        };
        let _ = store.exec_create(element_create).await?;
    }
    let file_node_update = FileNodeUpdate {
        children: None,
        parent_id: None,
        name: None,
        element_tree: Some(data.id.into()),
    };
    let filter: BTreeMap<String, Value> =
        BTreeMap::from([("id".into(), Value::Uuid(file_id.into()))]);
    // creating the tree
    store.exec_create(data).await?;
    // updating the file and setting the correct id in file_node
    store
        .exec_update(
            FileNode::table_name(),
            file_node_update,
            Some(filter.into()),
        )
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_element_tree(id: Uuid, ctx: State<'_, Context>) -> Result<ElementTree> {
    let store = ctx.get_store();
    let res = store
        .exec_get::<ElementTree>(Some(id.to_string()), Some(""))
        .await?
        .remove(0);
    Ok(res.try_into()?)
}

#[tauri::command]
pub async fn create_element(data: EditorElementCreate, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let parent_id = data.parent_id;
    let id = data.id;
    let tree_id = data.tree_id;
    let _ = store.exec_create(data).await?;
    let sql = "update $tb set children += $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((EditorElement::table_name(), parent_id.to_string()).into()),
        "va".into() => Value::Thing((EditorElement::table_name(), id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;

    let sql = "update $tb set elements.vertices += $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((ElementTree::table_name(), tree_id.to_string()).into()),
        "va".into() => Value::Thing((EditorElement::table_name(), id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn update_element(data: EditorElementUpdate, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let id = data.id;
    store
        .exec_update(
            EditorElement::table_name(),
            data,
            Some(BTreeMap::from([("id".into(), id.into())]).into()),
        )
        .await?;
    Ok(())
}
