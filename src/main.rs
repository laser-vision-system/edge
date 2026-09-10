use edge::conductor::Conductor;

#[tokio::main]
async fn main() {
	let _result = Conductor::new().await.run().await;
}
