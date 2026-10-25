//! Entrypoint for the Edge system.

use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync>> {
	edge::run().await
}
