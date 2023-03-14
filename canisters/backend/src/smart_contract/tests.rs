#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use crate::{create_nft, export_candid, get_nft_data, Row};
    use crate::initialize::init;

    #[test]
    fn save_candid() {
        write("smart.did", export_candid()).expect("Write failed.");
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
