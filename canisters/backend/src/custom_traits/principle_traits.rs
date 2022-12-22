use ic_cdk::export::Principal;

pub trait PrincipalCustomTraits {
    fn is_anonymous() -> bool;
}

impl PrincipalCustomTraits for Principal {
    fn is_anonymous() -> bool {
        let _caller: Principal = ic_cdk::caller();
        return _caller.to_text() == "2vxsx-fae".to_string();
    }
}
