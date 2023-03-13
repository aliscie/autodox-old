use candid::candid_method;
use ic_kit::macros::query;
use ic_stable_memory::s;

use crate::NftCollection;
use crate::NftData;

#[query]
#[candid_method(query)]
pub fn get_nft_data(address: String) -> Option<NftData> {
    let nfts = s!(NftCollection);

    for nft in nfts {
        if nft.address == address {
            return Some(nft);
        }
    }
    None
}

#[query]
#[candid_method(query)]
pub fn get_nft_collection() -> NftCollection {
    let nfts = s!(NftCollection);

    let mut query_nfts = Vec::new();
    for nft in nfts {
        query_nfts.push(NftData {
            address: nft.address,
            name: nft.name,
            data: nft.data,
            created_time: nft.created_time,
            owner_address: nft.owner_address,
        });
    }
    query_nfts
}
