use crate::nft::{create_nft, get_nft_data};
use crate::nft::{Column, NftData, Row, NftCollection};
use crate::initialize::init;

use candid::export_service;
use ic_kit::macros::query;

#[query(name = "__get_candid_interface")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn save_candid() {
        write("smart_contract.did", export_candid()).expect("Write failed.");
    }

    #[test]
    fn row_test() {
        println!(">>>>>>>>>> row_test <<<<<<<<<");
        init();

        let nft_u = create_nft(
            "0x1000".to_string(),
            "autodox_1".to_string(),
            "image_01".to_string(),
            "0x4000".to_string(),
        );

        assert_ne!(nft_u, None);
        let mut nft = nft_u.unwrap();
        
        let row: Row = vec!["a".to_string(), "b".to_string()];

        nft.add_row(row.clone());

        println!("row {:?}", row);

        assert_ne!(row.len(), 0);

        println!("{:?}", nft);

        write("smart.did", export_candid()).expect("Write failed.");
    }

    //#[test]
    fn create_nft_test() {
        println!(">>>>>>>>>> create_nft_test <<<<<<<<<");
        init();

        let nft = create_nft(
            "0x1000".to_string(),
            "autodox_1".to_string(),
            "image_01".to_string(),
            "0x4000".to_string(),
        );

        assert_ne!(nft, None);
        println!("{:?} ===================", nft.unwrap());

        let nft1 = get_nft_data("0x1000".to_string());
        
        assert_ne!(nft1, None);        
    }

}
