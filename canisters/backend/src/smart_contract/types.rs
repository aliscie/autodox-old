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

    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

pub type Row = Vec<String>;

#[derive(Clone, Readable, Writable, Deserialize, CandidType, Debug, Hash)]
pub struct Column {
    name: String,
    formula: String,
    filters: Vec<Filter>,
}

#[derive(Clone, Readable, Writable, Deserialize, CandidType, Debug, Hash)]
pub struct Filter {
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

pub fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.formula == other.formula
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.formula == other.formula &&
        do_vecs_match::<Filter>(&self.filters, &other.filters)
    }
}
