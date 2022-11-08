extern crate futures;
extern crate shared;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate yew;

use std::env;

use dotenv::dotenv;
use futures::executor;
use ic_cdk;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;
use lazy_static::lazy_static;
use reqwest;
use wasm_bindgen_futures::spawn_local;
use web_sys;

use app::App;

mod app;
mod app_components;
mod backend;
mod components;
mod router;
mod test;
mod utils;

lazy_static! {
    pub static ref IS_WEB: bool = {
        let mut m: bool = false;
        #[cfg(feature = "web")]
        {
            m = true;
        }
        m
    };
    pub static ref IS_LOGEDIN: bool = false;
}

// #[allow(unused_variable)]
fn main() {
    // spawn_local(async move {

    // let params = [("method", "greet"), ("name", "ali")];
    // let body = reqwest::post("http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai")
    //     .form(&params)
    //     .send()
    //     .await;

    // dotenv().ok();
    // let BACKEND_CANISTER: &str = &std::env::var("BACKEND_CANISTER").expect("BACKEND_CANISTER is not set");
    // let BACKEND_CANISTER = "ug5r5-74qxz-4dkqw-bjx6f-2wlit-3pqay-unom3-memsb-xgv4t-ljf2z-sae";  // The management canister ID.
    // let principal = Principal::from_text(BACKEND_CANISTER).expect("Could not decode the principal.");
    // let greeting: = ic_cdk::call(principal, "greet", ("Ali", "" )).await;
    // });

    yew::start_app::<App>();
}
