mod backend;
mod components;
mod app_components;
mod extensions;
mod test;
mod utils;
mod router;
mod app;

use app::App;

use lazy_static::lazy_static;
use ic_cdk;
use ic_cdk::export::Principal;

lazy_static! {
    pub static ref IS_WEB: bool = {
        let mut m : bool = false;
        #[cfg(feature = "web")] {
            m = true;
        }
        m
    };
    pub static ref IS_LOGEDIN: bool = false;
}

fn main() {
    async move {
        let text = "aaaaa-aa";  // The management canister ID.
        let principal = Principal::from_text(text).expect("Could not decode the principal.");
        // let greeting = ic_cdk::api::call::call(principal, "greet", ("Ali", )).await;
    };


    yew::start_app::<App>();
}

