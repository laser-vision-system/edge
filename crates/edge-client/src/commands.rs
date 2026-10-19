use std::collections::HashMap;
use api::edge::{EdgeRequests, NewCameraInfo, Settings};
use api::shared::{AlertType, CameraId, CameraStatus, ConnId, Deviation};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::oneshot;
use crate::error::EdgeClientResult;
use crate::websocket_requests::CameraRequest;

///Commands used internally by the client handler and the client to call functions across threads
pub enum Command {

    // Thread commands:

    SubscribeNewCamera {
        camera_id: CameraId,
        res_tx: oneshot::Sender<EdgeClientResult<UnboundedReceiver<CameraRequest>>>
    },

    SubscribeNewListener {
        res_tx: oneshot::Sender<EdgeClientResult<UnboundedReceiver<EdgeRequests>>>
    },

    // Websocket commands:

    RegisterCameras {
        cameras: HashMap<CameraId, NewCameraInfo>
    },

    UpdateCameraStatus {
        camera_id: CameraId,
        status: CameraStatus,
    },

    ImageAnalysis {
        camera_id: CameraId,
        deviation: Deviation,
    },

    Configuration {
        conn_id: ConnId,
        camera_id: CameraId,
        configuration: String,
    },

    ForgetCameraRequest{
        conn_id: ConnId,
        camera_id: CameraId,
        confirm: bool,
    },

    Alert {
        camera_id: Option<CameraId>,
        alert_type: AlertType,
        alert_code: u32,
        alert_name: String,
        alert_description: String,
    },

    SetNameCamera {
        camera_id: CameraId,
        name: String,
    },

    SendImage {
        camera_id: CameraId,
        frame: Vec<u8>
    },

    // Miscellaneous commands

    GetDefaultSettings {
        res_tx: oneshot::Sender<EdgeClientResult<Settings>>,
    },

    GetServerURL {
        res_tx: oneshot::Sender<EdgeClientResult<String>>,
    }
}