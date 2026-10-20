use api::edge::EdgeRequests;
use api::shared::VisionTechnique;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::interface::websocket_requests::{CameraRequest, PersistenceRequest};

/// Receiver for the status of the websocket, with true
/// indicating the websocket has come up, and false indicating
/// the websocket has gone down. Any iteration through the
/// `.next()` method will be in chronological order.
pub type StatusRx = UnboundedReceiver<bool>;

pub type StatusTx = UnboundedSender<bool>;

pub type VisionRx = UnboundedReceiver<VisionTechnique>;
pub type VisionTx = UnboundedSender<VisionTechnique>;

pub type ListenerRx = UnboundedReceiver<EdgeRequests>;
pub type ListenerTx = UnboundedSender<EdgeRequests>;

pub type PersistenceRx = UnboundedReceiver<PersistenceRequest>;
pub type PersistenceTx = UnboundedSender<PersistenceRequest>;

pub type CameraRx = UnboundedReceiver<CameraRequest>;
pub type CameraTx = UnboundedSender<CameraRequest>;