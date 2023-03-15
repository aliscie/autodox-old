use crate::elements::types::ElementTrees;
use crate::users::types::Users;
use crate::users::types::{User, UserFiles};
use ic_kit::candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use shared::{id::Id, schema::ElementTree, schema::*, Tree};
use std::collections::HashMap;

#[query]
#[candid_method(query)]
pub fn get_element_trees() -> Result<String, String> {
    ic_cdk::println!("Timer canister: Counter: {}", 1);
    let user = User::current().expect("No user found");
    let element_trees: ElementTrees = s!(ElementTrees);
    let r = element_trees
        .get(&user)
        .cloned()
        .ok_or_else(|| "User has no elements.".to_string())?;
    Ok(serde_json::to_string(&r).unwrap())

    // Ok(element_trees.get(&user).unwrap().clone())
    // TODO
    //     panicked at 'unexpected exception: JsValue(Error: Cannot find field name
    //     Error: Cannot find field name
    //         at Y.decodeValue (<anonymous>:8:41353)
    //         at J.decodeValue (<anonymous>:8:42418)
    //         at U.decodeValue (<anonymous>:8:39117)
    //         at Y.decodeValue (<anonymous>:8:41301)
    //         at Y.decodeValue (<anonymous>:8:41301)
    //         at J.decodeValue (<anonymous>:8:42418)
    //         at U.decodeValue (<anonymous>:8:39117)
    //         at L.decodeValue (<anonymous>:8:43636)
    //         at <anonymous>:8:50708
    //         at Array.map (<anonymous>))', frontend/src/backend/mod.rs:15:1
    //
    //     Stack:
}

#[query]
#[candid_method(query)]
pub fn get_element_tree(id: String) -> Result<String, String> {
    let user = User::current().expect("No user found");
    let element_trees: ElementTrees = s!(ElementTrees);
    let id: Id = id.try_into().unwrap();
    let r = element_trees
        .get(&user)
        .and_then(|tree_map| tree_map.get(&id));
    Ok(serde_json::to_string(&r).unwrap())
}
