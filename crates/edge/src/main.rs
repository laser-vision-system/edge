use std::error;
use tokio::try_join;
use edge_client::handler::client_handler::ClientHandler;
use edge_client::client::client::Client;
use edge_client::client::client_interface::ClientInterface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	let (
		client,
		status_rx,
		vision_rx,
		persistence_rx,
		mut thread
	) =  ClientHandler::new::<Client>().await;

	let thread = thread.run();

	try_join!(
		async move { edge::run().await },
		async move { thread.await }
	)
}
