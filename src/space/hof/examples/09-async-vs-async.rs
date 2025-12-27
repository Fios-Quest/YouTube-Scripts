#[tokio::main]
async fn main() {
    let async_closure_new = async || "Hello".to_string();

    let async_closure_old = || async { "Hello".to_string() };

    assert_eq!("Hello".to_string(), async_closure_new().await);
    assert_eq!("Hello".to_string(), async_closure_old().await);
}
