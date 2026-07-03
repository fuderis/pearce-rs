#![cfg(all(feature = "client", feature = "stream"))]
use pearce::{Client, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::tcp();

    let mut stream = client
        .post("http://127.0.0.1:8080/wait")
        .stream::<String>()
        .await
        .unwrap();

    while let Some(chunk) = stream.recv().await? {
        println!("{chunk}");
    }

    Ok(())
}
