use std::error;

use edge_client::client::client_interface::ClientInterface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	edge::run().await
}
