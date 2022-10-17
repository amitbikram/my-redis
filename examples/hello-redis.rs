use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut c = client::connect("127.0.0.1:6379").await?;
    c.set("hello", "world".into()).await?;
    let result = c.get("hello").await?;
    println!("got the value from server: {:?}", result);
    Ok(())
}
