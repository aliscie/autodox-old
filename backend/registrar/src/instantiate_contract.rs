use ic_kit::{
    Principal,
    candid::{CandidType, Nat, Encode, Deserialize},
    ic, interfaces::management
};

const STORAGE_WASM: &[u8] = std::include_bytes!("../../wasm_file/storage.wasm");

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}

#[derive(CandidType)]
pub enum InstantiateError{
    FailedToInstantiateCanister,
    FailedToInstallCode,
    FailedToSerialize,
}

async fn get_an_address() -> Principal{
    let canister_setting = management::CanisterSettings{
        controllers: Some(vec![ic::id()]),
        compute_allocation: Some(Nat::from(0_u64)),
        memory_allocation: Some(Nat::from(0)),
        freezing_threshold: Some(Nat::from(0)),
    };
    let args = management::CreateCanisterArgument{
        settings: Some(canister_setting)
    };
    let (canister_id,): (CanisterIdRecord,) = match ic::call_with_payment(Principal::management_canister(), "create_canister", (args,), 4_000_000_000_000).await
    {
        Ok(x) => x,
        Err((_, _)) => (CanisterIdRecord{
            canister_id: Principal::anonymous()
        },),
    };
    canister_id.canister_id
}

async fn install_wasm(wasm: &[u8], canister_id: Principal, args: Vec<u8>,) -> bool{
    let install_config = management::InstallCodeArgument{
        mode: management::InstallMode::Install,
        wasm_module: wasm.to_vec(),
        canister_id,
        arg: args
    };
    match ic::call(Principal::management_canister(), "install_code", (install_config,)).await
    {
        Ok(x) => x,
        Err((_, _)) =>{
            return false
        }
    }
    true
}

#[derive(CandidType)]
struct InitData{
    owner: Principal
}

pub async fn create_storage(caller: &Principal) -> Result<Principal, InstantiateError>{
    let canister_address = get_an_address().await;
    if canister_address == Principal::anonymous(){
        return Err(InstantiateError::FailedToInstantiateCanister)
    }
    let init_data = InitData{owner: caller.clone()};
    let args = Encode!(&init_data).map_err(|_| InstantiateError::FailedToSerialize)?;
    let res = install_wasm(STORAGE_WASM, canister_address, args).await;
    if res{
        Ok(canister_address)
    }else{
        Err(InstantiateError::FailedToInstallCode)
    }
}