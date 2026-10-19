use api::shared::VisionTechnique;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::commands::Command;
use crate::websocket_requests::PersistenceRequest;

/// An object that can be used to handle sending and receiving messages from a websocket on a separate thread.
pub trait ClientInterface {
    async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        internal_status_tx: UnboundedSender<bool>,
        status_tx: UnboundedSender<bool>,
        vision_tx: UnboundedSender<VisionTechnique>,
        persistence_tx: UnboundedSender<PersistenceRequest>,
    ) -> Self;

    /// Starts the client thread loop
    async fn run(mut self);
}