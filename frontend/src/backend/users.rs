use serde::Deserialize;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
#[derive(Deserialize, Debug)]
pub struct QueryUser {
    // #[wasm_bindgen(skip)]
    pub image: Option<Vec<u8>>,

    // #[wasm_bindgen(skip)]
    pub username: Option<String>,
}
