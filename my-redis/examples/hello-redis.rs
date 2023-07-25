use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis addess.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    let _ = client.set("hello", "world".into()).await;
    let out = client.get("hello").await;
    println!("{:?}", out);

    Ok(())
}
