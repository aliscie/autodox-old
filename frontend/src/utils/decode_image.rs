use js_sys::{Promise, Uint8Array};
use web_sys::{BlobPropertyBag, File, Url};
use web_sys::{Blob};

pub fn decode_image(image: Vec<u8>) -> Option<String> {
    let image = Uint8Array::from(image.as_slice());
    let mut optoins = BlobPropertyBag::new();
    let mut optoins = optoins.type_("image/png");
    let blob = Blob::new_with_u8_array_sequence_and_options(&image, &optoins).unwrap();
    let url: String = Url::create_object_url_with_blob(&blob).unwrap();
    // let image_url = Blob::new_with_buffer_source_sequence_and_options(&vec![bytes], "image/png").unwrap();
    Some(url)
}