use crate::context::Context;
use crate::prelude::*;
use crate::utils::*;
use indexmap::IndexSet;
use shared::id::Id;
use shared::schema::*;
use shared::traits::Entity;
use std::collections::BTreeMap;
use surrealdb::sql::{Thing, Value};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_element_tree(
    file_id: Id,
    mut data: ElementTree,
    ctx: State<'_, Context>,
) -> Result<()> {
    let store = ctx.get_store();
    for (id, i) in &data.elements.vertices {
        let children = data.elements.adjacency.entry(*id).or_default().clone();
        let element_create = EditorElementCreate {
            id: *id,
            text: i.text.clone(),
            children: Some(children),
            attrs: i.attrs.clone(),
            // these doesn't matter we are throwing
            parent_id: Uuid::new_v4().into(),
            tree_id: Uuid::new_v4().into(),
            prev_element_id: None,
        };
        let _ = store.exec_create(element_create).await?;
    }
    println!("file_id is : {:?}", file_id);
    let file_node_update = FileNodeUpdate {
        id: file_id,
        children: None,
        parent_id: None,
        name: None,
        element_tree: Some(data.id.into()),
    };
    // creating the tree
    store.exec_create(data).await?;
    // updating the file and setting the correct id in file_node
    store
        .exec_update(
            Thing::from((FileNode::table_name(), file_id.to_string())),
            file_node_update,
            None,
        )
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_element_tree(id: Uuid, ctx: State<'_, Context>) -> Result<ElementTree> {
    let store = ctx.get_store();
    let res = store
        .exec_get::<ElementTree>(Some(id.to_string()), Some("elements.vertices.*.*"))
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
    match data.prev_element_id {
        Some(prev_element_id) => {
            let mut children = load_children_ids(parent_id, &store).await?;
            children.move_index(
                children.get_index_of(&id).unwrap(),
                children.get_index_of(&prev_element_id).unwrap() + 1,
            );
            let editor_element_update = EditorElementUpdate {
                children: Some(children),
                id: parent_id,
                // ..Default::default()
                text: None,
                attrs: None,
                parent: None,
            };
            store
                .exec_update(
                    Thing::from((EditorElement::table_name(), parent_id.to_string())),
                    editor_element_update,
                    None,
                )
                .await?;
        }
        None => {
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
        }
    }

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
            Thing::from((EditorElement::table_name(), id.to_string())),
            data,
            None,
        )
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_element(data: EditorElementDelete, ctx: State<'_, Context>) -> Result<()> {
    let store = ctx.get_store();
    let mut stack = vec![data.id];
    let mut current_index = 0;
    while current_index < stack.len() {
        let children: Vec<Value> = store
            .exec_delete::<EditorElement>(stack[current_index].to_string())
            .await?
            .remove("children")
            .ok_or(Error::XPropertyNotFound("children".into()))?
            .try_into()?;
        for i in children {
            match i {
                Value::Thing(x) => {
                    let id = Uuid::parse_str(x.id.to_string().as_str());
                    if let Ok(id) = id {
                        stack.push(id.into());
                    }
                }
                _ => continue,
            }
        }
        current_index += 1;
    }
    // deleting node from the parent_element
    let sql = "update $tb set children -= $va";
    let vars: BTreeMap<String, Value> = map![
        "tb".into() => Value::Thing((EditorElement::table_name(), data.parent_id.to_string()).into()),
        "va".into() => Value::Thing((EditorElement::table_name(), data.id.to_string()).into()),
    ];
    store
        .datastore
        .execute(&sql, &store.session, Some(vars), false)
        .await?;
    // deleting all nodes from the element_tree.vertices column
    let sql = "update $tb set elements.vertices -= $va";
    for i in stack {
        let vars: BTreeMap<String, Value> = map![
            "tb".into() => Value::Thing((ElementTree::table_name(), data.tree_id.to_string()).into()),
            "va".into() => Value::Thing((EditorElement::table_name(), i.to_string()).into()),
        ];
        store
            .datastore
            .execute(&sql, &store.session, Some(vars), false)
            .await?;
    }
    Ok(())
}

async fn load_children_ids(parent_id: Id, store: &Store) -> Result<IndexSet<Id>> {
    let children_value: Vec<Value> = store
        .exec_select_only::<EditorElement>(parent_id.to_string(), &["children"])
        .await?
        .remove("children")
        .ok_or(Error::XPropertyNotFound("children".into()))?
        .try_into()?;
    let mut children: IndexSet<Id> = IndexSet::new();
    for i in children_value {
        match i {
            Value::Thing(x) => {
                children.insert(
                    Uuid::parse_str(x.id.to_string().as_str())
                        .map_err(|_| {
                            Error::XValueNotOfType("create_element value not of type Uuid")
                        })?
                        .into(),
                );
            }
            _ => {
                return Err(Error::XValueNotOfType(
                    "create_element Value not of type Thing",
                ))
            }
        }
    }
    Ok(children)
}
