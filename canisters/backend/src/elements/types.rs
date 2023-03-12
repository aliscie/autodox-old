use shared::{id::Id, schema::ElementTree};
use std::collections::HashMap;
use crate::users::types::User;

// pub type ElementTrees = HashMap<Id, ElementTree>;
pub type ElementTrees = HashMap<User, HashMap<Id, ElementTree>>;
