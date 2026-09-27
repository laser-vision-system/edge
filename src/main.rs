
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	println!("hello world!");
	edge::run().await
}
