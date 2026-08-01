use edge::Conductor;
use tokio;

#[tokio::main]
async fn main() {
	Conductor::new().await.run().await;
}
