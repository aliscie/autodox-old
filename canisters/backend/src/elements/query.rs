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
pub fn get_element_trees() -> Option<HashMap<Id, ElementTree>> {
    let user = User::current()?;
    let element_trees: ElementTrees = s!(ElementTrees);
    element_trees.get(&user).map(|s| s.clone())
}