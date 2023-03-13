use candid::candid_method;
use ic_kit::macros::update;
use std::time::{SystemTime, UNIX_EPOCH};

use ic_stable_memory::s;

use crate::get_nft_data;
use crate::NftCollection;
use crate::NftData;

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

    Some(NftData {
        address,
        name: Some(name),
        data: Some(data),
        owner_address: Some(owner_address),
        created_time: Some(now),
        [], []
    })
}

pub fn update_nft_owner(mut nft: NftData, owner_address: Option<String>) -> bool {
    nft.owner_address = owner_address;
    true
}

pub fn find_update_nft_owner(address: String, owner_address: Option<String>) -> bool {
    match get_nft_data(address) {
        None => false,
        Some(nft) => {
            return update_nft_owner(nft, owner_address);
        }
    }
}
