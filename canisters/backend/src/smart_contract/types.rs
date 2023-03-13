use candid::CandidType;
use serde::Deserialize;
use speedy::{Readable, Writable};

#[derive(Clone, Readable, Writable, Deserialize, CandidType, Debug, Hash)]
pub struct NftData {
    pub address: String,
    pub name: Option<String>,
    pub created_time: Option<u64>,
    pub owner_address: Option<String>,
    pub data: Option<String>,

    columns: Vec<Column>,
    rows: Vec<Row>,
}

type Row = Vec<String>;

#[derive(Clone, Readable, Writable, Deserialize, CandidType, Debug, Hash)]
struct Column {
    name: String,
    formula: String,
    filters: Vec<Filter>,
}

#[derive(Clone, Readable, Writable, Deserialize, CandidType, Debug, Hash)]
struct Filter {
    name: String,
    formula: String,
}

pub type NftCollection = Vec<NftData>;

impl Eq for NftData {}

impl PartialEq for NftData {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
