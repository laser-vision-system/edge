use std::collections::HashMap;
use api::edge::{EdgeRequests, NewCameraInfo, Settings};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::commands::Command;
use api::shared::{AlertType, CameraId, CameraStatus, ConnId, Deviation, VisionTechnique};
use tokio::spawn;
use tokio::sync::oneshot;
use edge_core::logger::logger;
use crate::client_interface::ClientInterface;
use crate::error::{EdgeClientError, EdgeClientResult};
use crate::websocket_requests::{CameraRequest, PersistenceRequest};

/// Interface for other threads to access the websocket client.
///
/// Contains all functions necessary to communicate with the websocket,
/// and to get receivers for messages from the websocket thread.
pub struct ClientHandler<Backend = dyn ClientInterface> {
    client: Backend,
    cmd_tx: UnboundedSender<Command>,
    status: bool,
    internal_status_rx: UnboundedReceiver<bool>
}

impl ClientHandler<dyn ClientInterface> {

    /// Starts a websocket client on a separate thread.
    /// Returns itself, for communications with the thread,
    /// as well as a receiver which sends true when the websocket comes online,
    /// a receiver to indicate vision technique change requests,
    /// and a receiver for persistence specific requests.
    ///
    /// The client expects these receivers to never be dropped,
    /// and will panic upon sending data to them if they are.
    pub async fn new() -> (ClientHandler, UnboundedReceiver<bool>, UnboundedReceiver<VisionTechnique>, UnboundedReceiver<PersistenceRequest>) {
        let (cmd_tx, cmd_rx) = unbounded_channel();
        let (status_tx, status_rx) = unbounded_channel();
        let (internal_status_tx, internal_status_rx) = unbounded_channel();
        let (vision_tx, vision_rx) = unbounded_channel();
        let (persistence_tx, persistence_rx) = unbounded_channel();
        let client = ClientInterface::new(cmd_rx, internal_status_tx, status_tx, vision_tx, persistence_tx).await;

        spawn(client.run());

        logger()
            .lock()
            .unwrap()
            .log_info("Websocket client thread started");

        (
            ClientHandler{
                client,
                cmd_tx,
                status: false,
                internal_status_rx,
            },
            status_rx,
            vision_rx,
            persistence_rx,

        )
    }

    pub async fn status(&mut self) -> bool {
        let _ = self.update_status().await;

        self.status
    }

    /// Updates 'self.status' to the current state of the websocket.
    async fn update_status(&mut self) -> EdgeClientResult<()>{
        while let Ok(status) = self.internal_status_rx.try_recv() {
            self.status = status;
        }

        if self.status {
            return Ok(())
        }
        Err(EdgeClientError::WebsocketDown)
    }

    /// Gets the event receiver for a new camera.
    ///
    /// These receivers can be dropped without issue.
    pub async fn subscribe_new_camera(&mut self, camera_id: CameraId)
        -> EdgeClientResult<UnboundedReceiver<CameraRequest>> {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::SubscribeNewCamera {
                camera_id,
                res_tx
            }).expect("Client thread closed");

        res_rx.await.expect("Return handler dropped")
    }

    /// Gets a generic listener for handling any websocket request.
    ///
    /// Will be phased out as the other receivers are put into place.
    ///
    /// These receivers can be dropped without issue.
    pub async fn subscribe_new_listener(&mut self)
        -> EdgeClientResult<UnboundedReceiver<EdgeRequests>> {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::SubscribeNewListener {
                res_tx
            }).expect("Client thread closed");

        res_rx.await.expect("Return handler dropped")
    }

    /// Sends the RegisterCameras command to the server
    pub async fn register_cameras(&mut self, cameras: HashMap<CameraId, NewCameraInfo>)
        -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::RegisterCameras { cameras })
            .expect("Client thread closed");
        Ok(())
    }

    pub async fn update_camera_status(&mut self, camera_id: CameraId, status: CameraStatus)
        -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::UpdateCameraStatus { camera_id, status })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn image_analysis (&mut self, camera_id: CameraId, deviation: Deviation)
        -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::ImageAnalysis { camera_id, deviation })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn configuration (
        &mut self,
        conn_id: ConnId,
        camera_id: CameraId,
        configuration: String
    ) -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::Configuration {
                conn_id,
                camera_id,
                configuration
            })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn confirm_forget_camera(
        &mut self,
        conn_id: ConnId,
        camera_id: CameraId,
        confirm: bool
    ) -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::ForgetCameraRequest {
                conn_id,
                camera_id,
                confirm
            })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn alert (
        &mut self,
        camera_id: Option<CameraId>,
        alert_type: AlertType,
        alert_code: u32,
        alert_name: String,
        alert_description: String,
    ) -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::Alert {
                camera_id,
                alert_type,
                alert_code,
                alert_name,
                alert_description,
            })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn set_name_camera(&mut self, camera_id: CameraId, name: String)
        -> EdgeClientResult<()> {
        self.update_status().await?;

        self.cmd_tx
            .send(Command::SetNameCamera { camera_id, name })
            .expect("Client thread closed");

        Ok(())
    }

    pub async fn get_default_setting(&mut self) -> EdgeClientResult<Settings> {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::GetDefaultSettings {res_tx})
            .expect("Client thread closed");

        res_rx.await.expect("Return handler dropped")
    }

    pub async fn get_server_url(&mut self) -> EdgeClientResult<String> {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send(Command::GetServerURL {res_tx})
            .expect("Client thread closed");

        res_rx.await.expect("Return handler dropped")
    }
}