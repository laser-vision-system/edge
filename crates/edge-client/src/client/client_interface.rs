use tokio::sync::mpsc::{
    UnboundedSender,
};
use crate::interface::commands::Command;
use crate::interface::types::{PersistenceRx, StatusRx, VisionRx};

/// An object that can be used to handle sending and receiving messages from a
/// websocket on a separate thread.
pub trait ClientInterface {
	fn new() -> (
		UnboundedSender<Command>,
		StatusRx,
		StatusRx,
		VisionRx,
		PersistenceRx,
		Self,
	);

	/// Starts the client thread loop
	async fn run(&mut self);
}
