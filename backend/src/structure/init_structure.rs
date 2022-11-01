use ic_stable_memory::{collections::hash_map::SHashMap, utils::ic_types::SPrincipal};

// use ic_kit::candid::CandidType;

// #[derive(CandidType)]
// pub struct Content{
//     pub parent_id: Option<String>,
//     pub child_id: String,
//     pub content: String,
// }

pub type Storage = SHashMap<SPrincipal, SHashMap<String, SHashMap<String, String>>>;
