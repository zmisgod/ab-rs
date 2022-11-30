mod fetch;
mod http;

#[tokio::main]
async fn main() {
    fetch::run().await;
}