use crate::elements::types::ElementTrees;
use shared::{id::Id, schema::*, schema::ElementTree, Tree};
use crate::users::types::Users;
use crate::users::types::{User, UserFiles};
use ic_kit::candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use std::collections::HashMap;

#[query]
#[candid_method(query)]
pub fn get_element_trees() -> Result<HashMap<Id, ElementTree>, String> {
    ic_cdk::print!("get_element_trees");
    let user = User::current().expect("No user found");
    let element_trees: ElementTrees = s!(ElementTrees);
    element_trees.get(&user).cloned().ok_or_else(|| "User has no elements.".to_string())

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