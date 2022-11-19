extern crate futures;
extern crate shared;
extern crate wasm_bindgen_futures;
extern crate yew;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use app::App;
use shared::log;

mod app;
mod app_components;
mod backend;
mod components;
mod router;
mod test;
mod utils;

use wasm_bindgen_futures::spawn_local;
use futures::executor::block_on;

lazy_static! {
    pub static ref IS_WEB: bool = cfg!(feature = "web");
    pub static ref IS_LOGEDIN: bool = {
        false
        // TODO
        // let mut is_logged: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
        // spawn_local(async move {
        //     is_logged.set_value(serde_wasm_bindgen::from_value::<bool>(backend::is_logged().await).unwrap();)
        // });
        // *is_logged
    };
}

#[wasm_bindgen]
pub fn run() {
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
