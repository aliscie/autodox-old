use serde::Deserialize;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
#[derive(Deserialize, Debug)]
pub struct UserQuery {
    // #[wasm_bindgen(skip)]
    pub image: Option<Vec<u8>>,

    // #[wasm_bindgen(skip)]
    pub username: Option<String>,
}
