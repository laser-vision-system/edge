use edge::Conductor;

#[tokio::main]
async fn main() {
	Conductor::new().await.run().await;
}
