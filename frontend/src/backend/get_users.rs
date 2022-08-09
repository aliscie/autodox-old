use serde::Deserialize;
use reqwest::Error;
use web_sys::console::log_1;
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    log_1(&format!("{:#?}", users).into());
    Ok(())
}
