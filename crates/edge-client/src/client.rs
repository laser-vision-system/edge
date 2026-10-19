//!

use std::collections::{HashMap, HashSet};
use api::shared::{AlertType, CameraStatus, Configuration, ConnId, Deviation, ForgetCameraRequest, VisionTechnique};
use api::edge::{Alert, EdgeRequests, ImageAnalysis, InitialiseConnection, NewCameraInfo, RegisterCameras, ServerRequests, SetNameCamera, Settings};
use api::shared::CameraId;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::FusedStream;
use reqwest::header::Entry::Vacant;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::interval;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{Bytes, Error, Message, Utf8Bytes};
use edge_core::logger::logger;
use edge_core::settings::{AUTO_CONNECT_DELAY, WS_KEEP_ALIVE};
use crate::client_interface::ClientInterface;
use crate::commands::Command;
use crate::auto_connect::get_ws_stream;
use crate::commands::Command::UpdateCameraStatus;
use crate::websocket_requests::{CameraRequest, PersistenceRequest};
use crate::error::{EdgeClientError, EdgeClientResult};
use crate::persistent_edge_id::get_mac_address;
use crate::websocket::WebSocket;

type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;


/// The main websocket client implementation.
#[derive(Debug)]
pub struct Client {
    cmd_rx: UnboundedReceiver<Command>,
    internal_status_tx: UnboundedSender<bool>,
    status_tx: UnboundedSender<bool>,
    vision_tx: UnboundedSender<VisionTechnique>,
    persistence_tx: UnboundedSender<PersistenceRequest>,
    cameras: HashMap<CameraId, UnboundedSender<CameraRequest>>,
    listeners: HashSet<UnboundedSender<EdgeRequests>>,
    url: Option<String>,
}

/// Methods for the clients overall process loop.
impl Client {
    /// TODO: write method to convert ws://*.*.*.*/edge_ws into *.*.*.*
    async fn convert_url(&mut self, url: String) -> String {
        "".to_string()
    }

    /// Loops until a websocket is successfully connected
    async fn connect_ws(&mut self) -> (WebsocketType, String) {
        let mut interval = interval(AUTO_CONNECT_DELAY);
        loop {
            interval.tick().await;
            if let (Ok(stream), url) = get_ws_stream() {
                return (stream, url)
            }
        }
    }

    async fn send_persistent_edge_id(&mut self, websocket: &mut WebsocketType) {
        let request_enum =
            ServerRequests::InitialiseConnection(InitialiseConnection {
                edge_id: get_mac_address().unwrap(),
            });
        self.send_text(websocket, request_enum).await;
    }

    async fn send_text<T>(
        &mut self,
        websocket: &mut WebsocketType,
        data: T,
    ) -> EdgeClientResult<()>
    where
        T: Serialize,
    {
        let result = websocket
            .send(Message::Text(Utf8Bytes::from(&serde_json::to_string(
                &data,
            )?)))
            .await;

        self.process_ws_err(result)
    }

    async fn process_ws_err(&mut self, result: Result<(), Error>) -> EdgeClientResult<()> {
        if result.is_err() {
            return match result.err() {
                None => {
                    logger()
                        .lock()
                        .unwrap()
                        .log_warn(
                            "Could not send text through websocket for unknown reason"
                        );
                    Ok(())
                }
                Some(err) => {
                    match err {
                        Error::ConnectionClosed => {
                            Err(EdgeClientError::WebsocketClosed)
                        }
                        Error::AlreadyClosed => {
                            Err(EdgeClientError::WebsocketClosed)
                        }

                        err => {
                            logger()
                                .lock()
                                .unwrap()
                                .log_warn(
                                    format!("Could not send text through websocket due to: {:?}", err).as_str()
                                );
                            Ok(())
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Forwards the SetProcessingType request to `vision_rx`
    async fn set_processing_type(&mut self, vision: VisionTechnique) {
        self.vision_tx.send(vision).expect("Vision channel closed");
    }

    /// Forwards non-vision-processing requests to their respective camera,
    /// logs an error if the request uses an invalid camera ID
    async fn camera_request(&mut self, request: CameraRequest, camera: CameraId) {
        let result =self.cameras.entry(camera)
            .and_modify(|entry| entry.send(request)?);

        match result {
            Vacant(_) => {
                logger()
                    .lock()
                    .unwrap()
                    .log_warn(&format!("Command received for invalid camera ID: {:?} ", camera));
            }

            _ => {}
        }
    }

    async fn persistence_request(&mut self, request: PersistenceRequest) {
        self.persistence_tx.send(request).expect("Persistence channel closed");
    }

    /// Forwards all requests to miscellaneous listeners. In place for migration purposes.
    async fn forward_request(&mut self, request: EdgeRequests) {
        for listener in self.listeners {
            let _ = listener.send(request.clone());
        }
    }

    async fn main_loop(&mut self, websocket: &mut WebsocketType) {
        let mut interval = interval(WS_KEEP_ALIVE);
        loop {
            tokio::select! {
                Some(Ok(cmd))  = self.cmd_rx.recv() => {
                    if let Err(e) = self.process_command(cmd, websocket).await {

                    }
                }

                Some(Ok(msg)) = websocket.next() => {
                    if let Err(e) = self.process_ws(msg).await {

                    }
                }

                _ = interval.tick() => {
                    if websocket.is_terminated() {
                        break
                    }
                    websocket.send(Message::Ping(Bytes::from("ping"))).await?
                }
            }
            websocket.send(Message::Close(None)).await?
        }
    }

    /// Handles Commands send by the client handler.
    async fn process_command(&mut self, command: Command, websocket: &mut WebsocketType) -> EdgeClientResult<()>  {
        match command {
            Command::SubscribeNewCamera {
                camera_id,
                res_tx
            } => {
                let result = self.subscribe_new_camera(camera_id);
                res_tx.send(result).expect("Return channel closed before returning");
            }

            Command::SubscribeNewListener
            { res_tx } => {
                let result = self.subscribe_listener().await;
                res_tx.send(result).expect("Return channel closed before returning");
            }

            Command::RegisterCameras
            { cameras } => {
                self.register_cameras(websocket, cameras).await?;
            }

            Command::UpdateCameraStatus {
                camera_id,
                status
            } => {
                self.update_camera_status(websocket, camera_id, status).await?;
            }

            Command::ImageAnalysis {
                camera_id,
                deviation
            } => {
                self.image_analysis(websocket, camera_id, deviation).await?;
            }

            Command::Configuration {
                conn_id,
                camera_id,
                configuration
            } => {
                self.configuration(websocket, conn_id, camera_id, configuration).await?;
            }

            Command::ForgetCameraRequest {
                conn_id,
                camera_id,
                confirm
            } => {
                self.forget_camera_request(websocket, conn_id, camera_id, confirm).await?;
            }

            Command::Alert {
                camera_id,
                alert_code,
                alert_type,
                alert_name,
                alert_description
            } => {
                self.alert(websocket, camera_id, alert_type, alert_code, alert_name, alert_description).await?;
            }

            Command::SetNameCamera {
                camera_id,
                name
            } => {
                self.set_name_camera(websocket, camera_id, name).await?;
            }

            Command::SendImage {
                camera_id,
                frame
            } => {
                self.send_image(websocket, camera_id, &mut frame.clone())?
            }

            Command::GetDefaultSettings
            { res_tx } => {
                let result = self.get_default_settings().await;
                res_tx.send(result).expect("Return channel closed before returning");
            }

            Command::GetServerURL { res_tx } => {
                let result = self.get_server_url().await;
                res_tx.send(result).expect("Return channel closed before returning");
            }

        }
        Ok(())
    }

    /// Processes a websocket message, returning an error on close.
    async fn process_ws(&mut self, msg: Message) -> EdgeClientResult<()> {
        match msg {
            Message::Text(text) => {
                self.process_message(&text).await;
            }

            Message::Close(_) => {
                return Err(EdgeClientError::WebsocketClosed)
            }

            _ => {}
        }
        Ok(())
    }

    /// Parses websocket requests and calls their respective functions.
    async fn process_message(&mut self, text: &str) {
        let msg = text.trim();

        let data_unchecked: serde_json::error::Result<EdgeRequests> = serde_json::from_str(msg);

        let Ok(request) = data_unchecked else {
            logger()
                .lock()
                .unwrap()
                .log_warn(
                    &format!("Could not deserialize websocket message due to: {:?}",
                             data_unchecked.err().unwrap().to_string())
                )
        };

        self.forward_request(request.clone()).await;

        match request {
            EdgeRequests::SetNameCamera(data) => {
                let request = CameraRequest::SetNameCamera(data.name);
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::ToggleViewingCamera(data) => {
                let request = CameraRequest::ToggleViewingCamera(data.start_viewing);
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::Configuration(data) => {
                let request = CameraRequest::Configuration(data.configuration);
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::ViewCameraConfig(data) => {
                let request = CameraRequest::ViewCameraConfig(data.conn_id);
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::Setpoint(data) => {
                let request = CameraRequest::Setpoint;
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::SetProcessingType(data) => {
                self.set_processing_type(data.processing_type).await;
            }

            EdgeRequests::ForceRetryCamera(data) => {
                let request = PersistenceRequest::ForceRetryCamera(data.camera_id);
                self.persistence_request(request).await;
            }
            EdgeRequests::ForgetCameraRequest(data) => {
                let request = PersistenceRequest::ForgetCameraRequest(data.conn_id, data.camera_id);
                self.persistence_request(request).await;
            }

            _ => {}
        }
    }
}

/// Methods for handling commands from the handler.
impl Client {
    async fn subscribe_new_camera(&mut self, camera_id: CameraId)
        -> EdgeClientResult<UnboundedReceiver<CameraRequest>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.cameras.insert(camera_id, tx);
        Ok(rx)
    }

    async fn subscribe_listener(&mut self) -> EdgeClientResult<UnboundedReceiver<EdgeRequests>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.listeners.insert(tx);
        Ok(rx)
    }

    async fn register_cameras(
        &mut self,
        websocket: &mut WebsocketType,
        cameras: HashMap<CameraId, NewCameraInfo>
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::RegisterCameras(RegisterCameras {
            cameras,
        });
        self.send_text(websocket, message).await
    }

    async fn update_camera_status(
        &mut self,
        websocket: &mut WebsocketType,
        camera_id: CameraId,
        status: CameraStatus,
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::UpdateCameraStatus(api::edge::UpdateCameraStatus {
            camera_id,
            status
        });
        self.send_text(websocket, message).await
    }

    async fn image_analysis(
        &mut self,
        websocket: &mut WebsocketType,
        camera_id: CameraId,
        deviation: Deviation
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::ImageAnalysis(ImageAnalysis {
            camera_id,
            deviation,
        });
        self.send_text(websocket, message).await
    }

    async fn configuration(
        &mut self,
        websocket: &mut WebsocketType,
        conn_id: ConnId,
        camera_id: CameraId,
        configuration: String,
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::Configuration(Configuration {
            conn_id,
            camera_id,
            configuration,
        });
        self.send_text(websocket, message).await
    }

    async fn forget_camera_request(
        &mut self,
        websocket: &mut WebsocketType,
        conn_id: ConnId,
        camera_id: CameraId,
        confirm: bool
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::ForgetCameraRequest(ForgetCameraRequest{
            conn_id,
            camera_id,
            confirm
        });
        self.send_text(websocket, message).await
    }

    async fn alert(
        &mut self,
        websocket: &mut WebsocketType,
        camera_id: Option<CameraId>,
        alert_type: AlertType,
        alert_code: u32,
        alert_name: String,
        alert_description: String,
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::Alert(Alert {
            camera_id,
            alert_type,
            alert_code,
            alert_name,
            alert_description
        });
        self.send_text(websocket, message).await
    }

    async fn set_name_camera(
        &mut self,
        websocket: &mut WebsocketType,
        camera_id: CameraId,
        name: String
    ) -> EdgeClientResult<()> {
        let message = ServerRequests::SetNameCamera(SetNameCamera {
            camera_id,
            name
        });
        self.send_text(websocket, message).await
    }

    pub async fn send_image(
        &mut self,
        websocket: &mut WebsocketType,
        camera_id: u64,
        frame: &mut Vec<u8>,
    ) -> EdgeClientResult<()> {
        let mut msg = camera_id.to_be_bytes().to_vec();
        msg.append(frame);
        let result = websocket.send(Message::Binary(Bytes::from(msg))).await;
        self.process_ws_err(result)
    }

    async fn get_default_settings(&mut self) -> EdgeClientResult<Settings> {
        crate::auto_connect::get_default_settings(
            self.url.expect("websocket URL not set while websocket connected")
                .as_str()).await
    }

    async fn get_server_url(&mut self) -> EdgeClientResult<String> {
        Ok(self.url.expect("websocket URL not set while websocket connected"))
    }
}

impl ClientInterface for Client {
    async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        internal_status_tx: UnboundedSender<bool>,
        status_tx: UnboundedSender<bool>,
        vision_tx: UnboundedSender<VisionTechnique>,
        persistence_tx: UnboundedSender<PersistenceRequest>,
    ) -> Self {
        Client {
            cmd_rx,
            internal_status_tx,
            status_tx,
            vision_tx,
            persistence_tx,
            cameras: HashMap::new(),
            listeners: HashSet::new(),
            url: None
        }
    }

    async fn run(mut self) {
        loop {
            let (mut websocket, url) = self.connect_ws().await;

            self.send_persistent_edge_id(&mut websocket).await;

            self.url = Some(self.convert_url(url)); // Saving the url here so eventually we can implement a get_default_settings method

            self.status_tx.send(true)
                .expect("Status receiver dropped before websocket thread was closed");
            self.internal_status_tx.send(true)
                .expect("Status receiver dropped before websocket thread was closed");

            logger()
                .lock()
                .unwrap()
                .log_info("Websocket connected");

            self.main_loop(&mut websocket).await;

            logger()
                .lock()
                .unwrap()
                .log_warn("Websocket disconnected");

            self.status_tx.send(false)
                .expect("Status receiver dropped before websocket thread was closed");
            self.internal_status_tx.send(false)
                .expect("Status receiver dropped before websocket thread was closed");
        }
    }
}

