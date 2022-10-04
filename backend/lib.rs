#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello from rust test 2, {}!", name)
}
