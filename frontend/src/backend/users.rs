use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryUser {
    pub username: String,
    pub image: u8,
}
