use candid::{candid_method, export_service};
use ic_kit::macros::update;
use std::time::{SystemTime, UNIX_EPOCH};

use ic_stable_memory::s;

use crate::nft::{do_vecs_match, get_nft_data, get_nft_collection};
use crate::nft::{Column, NftCollection, NftData, Row};

pub fn get_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[update]
#[candid_method(update)]
pub fn create_nft(
    address: String,
    name: String,
    data: String,
    owner_address: String,
) -> Option<NftData> {
    let now = get_secs();

    let found = get_nft_data(address.clone());

    if found != None {
        return found;
    }

    let nft: NftData = NftData {
        address,
        name: Some(name),
        data: Some(data),
        owner_address: Some(owner_address),
        created_time: Some(now),
        rows: Vec::new(),
        columns: Vec::new(),
    };

    let mut nfts = s!(NftCollection);
    nfts.push(nft.clone());
    s! {NftCollection = nfts};
    println!("collection={:?}", nfts);

    Some(nft)
}

#[update]
#[candid_method(update)]
pub fn update_nft_owner(mut nft: NftData, owner_address: Option<String>) -> bool {
    nft.owner_address = owner_address;
    true
}

#[update]
#[candid_method(update)]
pub fn find_update_nft_owner(address: String, owner_address: Option<String>) -> bool {
    match get_nft_data(address) {
        None => false,
        Some(nft) => {
            return update_nft_owner(nft, owner_address);
        }
    }
}

impl NftData {
    fn find_row(&mut self, row: Row) -> usize {
        let index = self
            .rows
            .iter()
            .position(|r1| do_vecs_match::<String>(r1, &row))
            .unwrap_or(0xFFFFFFFF);

        index
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn delete_row(&mut self, row: Row) -> bool {
        let idx = self.find_row(row);
        if idx == 0xFFFFFFF {
            return false;
        }
        self.rows.remove(idx);
        true
    }

    fn find_column(&mut self, column: Column) -> usize {
        let index = self
            .columns
            .iter()
            .position(|c1| *c1 == column)
            .unwrap_or(0xFFFFFFFF);

        index
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn delete_column(&mut self, column: Column) -> bool {
        let idx = self.find_column(column);
        if idx == 0xFFFFFFF {
            return false;
        }
        self.columns.remove(idx);
        true
    }
}
