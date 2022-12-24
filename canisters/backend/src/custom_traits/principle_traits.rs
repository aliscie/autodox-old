use ic_cdk::export::Principal;

pub trait PrincipalCustomTraits {
    fn is_anonymous() -> bool;

    // async fn rand_row() -> [u8; 16];
}

impl PrincipalCustomTraits for Principal {
    fn is_anonymous() -> bool {
        let _caller: Principal = ic_cdk::caller();
        return _caller.to_text() == "2vxsx-fae".to_string();
    }
    // async fn rand_row() -> [u8; 16] {
    //     ic_cdk::api::call::call(
    //         ic_cdk::export::Principal::management_canister(),
    //         "raw_rand",
    //         (),
    //     )
    //         .await
    //         .unwrap()
    // }
}
