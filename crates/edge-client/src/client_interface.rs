use api::shared::VisionTechnique;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::commands::Command;

/// An object that can be used to handle sending and receiving messages from a websocket on a separate thread.
pub trait ClientInterface {
    async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        status_tx: UnboundedSender<bool>,
        vision_tx: UnboundedSender<VisionTechnique>
    ) -> Self;

    /// Starts the client thread loop
    async fn run(mut self);
}