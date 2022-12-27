use js_sys::{Promise, Uint8Array};
use web_sys::{File};


pub async fn encode_image(image: File) -> Vec<u8> {
    let buffer: Promise = image.array_buffer();
    let result = wasm_bindgen_futures::JsFuture::from(buffer).await;
    let bytes: Vec<u8> = Uint8Array::new(&result.unwrap()).to_vec();
    bytes
}