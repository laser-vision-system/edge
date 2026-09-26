#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	edge::run().await
}
// god wont let me run the manual github actions so go go gadget comment on main