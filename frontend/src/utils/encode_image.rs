use js_sys::{Promise, Uint8Array};
use web_sys::File;
pub fn encode_image(image: File) -> Vec<u8>{
    let buffer = image.array_buffer().value_of();
    let bytes: Vec<u8> = Uint8Array::new(&buffer).to_vec();
    return bytes
}