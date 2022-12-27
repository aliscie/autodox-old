use web_sys::File;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub data: Vec<u8>,
}


// impl From<File> for Image {
//     fn from(image: File) -> Self {
//         let buffer = image.array_buffer();
//         let result = wasm_bindgen_futures::JsFuture::from(buffer).await;
//         let bytes: Vec<u8> = Uint8Array::new(&result.unwrap()).to_vec();
//         Self { width: 0, height: 0, format: "".to_string(), data: bytes }
//     }
// }
