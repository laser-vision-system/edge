use edge::conductor::Conductor;
use edge::client::Client;

#[tokio::main]
async fn main() {
	let _result = Conductor::new().await.run().await;
	// let mut client = Client::new();
	// client.identify_network();
}
