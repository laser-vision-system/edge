use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::commands::Command;
use api::shared::VisionTechnique;
use tokio::spawn;
use edge_core::logger::logger;
use crate::client::Client;
use crate::client_interface::ClientInterface;
use crate::websocket_requests::PersistenceRequest;

/// Interface for other threads to access the websocket client.
///
/// Contains all functions necessary to communicate with the websocket,
/// and to get receivers for messages from the websocket thread.
pub struct ClientHandler<Backend = dyn ClientInterface> {
    client: Backend,
    cmd_tx: UnboundedSender<Command>,
}

impl ClientHandler<dyn ClientInterface> {

    /// Starts a websocket client on a separate thread.
    /// Returns itself, for communications with the thread,
    /// as well as a receiver which sends true when the websocket comes online,
    /// a receiver to indicate vision technique change requests,
    /// and a receiver for persistence specific requests.
    ///
    /// Any of these receivers being dropped while the client thread is still up
    /// will cause the client thread to panic when it tries to send data to them.
    pub async fn new() -> (ClientHandler, UnboundedReceiver<bool>, UnboundedReceiver<VisionTechnique>, UnboundedReceiver<PersistenceRequest>) {
        let (cmd_tx, cmd_rx) = unbounded_channel();
        let (status_tx, status_rx) = unbounded_channel();
        let (vision_tx, vision_rx) = unbounded_channel();
        let (persistence_tx, persistence_rx) = unbounded_channel();
        let client = ClientInterface::new(cmd_rx, status_tx, vision_tx, persistence_tx).await;

        spawn(client.run());

        logger()
            .lock()
            .unwrap()
            .log_info("Websocket client thread started");

        (
            ClientHandler{
                client,
                cmd_tx,
            },
            status_rx,
            vision_rx,
            persistence_rx,
        )
    }
}