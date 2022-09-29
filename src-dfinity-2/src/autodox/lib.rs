#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Your name is, {}!", name)
}
