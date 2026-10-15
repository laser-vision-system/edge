use std::collections::{HashMap, HashSet};
use api::shared::VisionTechnique;
use api::edge::EdgeRequests;
use api::shared::CameraId;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::FusedStream;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::interval;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{Bytes, Message};
use edge_core::logger::logger;
use edge_core::settings::{AUTO_CONNECT_DELAY, WS_KEEP_ALIVE};
use crate::client_interface::ClientInterface;
use crate::commands::Command;
use crate::auto_connect::get_ws_stream;
use crate::camera_requests::CameraRequest;
use crate::error::EdgeClientError;

type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;


/// The main websocket client implementation.
#[derive(Debug)]
pub struct Client {
    cmd_rx: UnboundedReceiver<Command>,
    status_tx: UnboundedSender<bool>,
    vision_tx: UnboundedSender<VisionTechnique>,
    cameras: HashMap<CameraId, UnboundedSender<CameraRequest>>,
    listeners: HashSet<UnboundedSender<EdgeRequests>>,
    url: Option<String>
}

impl Client {

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

    /// Forwards the SetProcessingType request to `vision_rx`
    async fn set_processing_type(&mut self, vision: VisionTechnique) {
        self.vision_tx.send(vision).unwrap();
    }

    /// Forwards non-vision-processing requests to their respective camera,
    /// logs an error if the request uses an invalid camera ID
    // NOTE FOR REVIEW: are there any camera requests that shouldn't be directed to the cameras
    // themselves?
    async fn camera_request(&mut self, request: CameraRequest, camera: CameraId){
        if !self.cameras.contains_key(&camera) {
            logger()
                .lock()
                .unwrap()
                .log_warn(&format!("Command received for invalid camera ID: {:?} ", camera));
            return
        }

        let _ =self.cameras.get_mut(&camera).unwrap().send(request);
    }

    /// Forwards all requests to miscellaneous listeners. In place for migration purposes.
    async fn forward_request(&mut self, request: EdgeRequests) {
        for listener in self.listeners {
            let _ =listener.send(request.clone());
        }
    }

    async fn main_loop(&mut self, mut websocket: WebsocketType) {
        let mut interval = interval(WS_KEEP_ALIVE);
        loop {
            tokio::select! {
                Some(Ok(cmd))  = self.cmd_rx.recv() => {
                    if let Err(e) = self.process_command(cmd).await {

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
        }
    }

    /// Handles Commands send by the client handler.
    async fn process_command(&mut self, command: Command) -> Result<None, EdgeClientError>  {
        match command {

        }
    }

    /// Processes a websocket message, returning an error on close.
    async fn process_ws(&mut self, msg: Message) -> Result<None, EdgeClientError> {
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

            EdgeRequests::StartViewingCamera(data) => {
                let request = CameraRequest::StartViewingCamera;
                self.camera_request(request, data.camera_id).await;
            }

            EdgeRequests::StopViewingCamera(data) => {
                let request = CameraRequest::StopViewingCamera;
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
                let request = CameraRequest::ForceRetryCamera;
                self.camera_request(request, data.camera_id).await;
            }
            EdgeRequests::ForgetCameraRequest(data) => {
                let request = CameraRequest::ForgetCameraRequest(data.conn_id);
                self.camera_request(request, data.camera_id).await;
            }

            _ => {}
        }
    }
}

impl ClientInterface for Client {
    async fn new(
        cmd_rx: UnboundedReceiver<Command>,
        status_tx: UnboundedSender<bool>,
        vision_tx: UnboundedSender<VisionTechnique>
    ) -> Self {
        Client {
            cmd_rx,
            status_tx,
            vision_tx,
            cameras: HashMap::new(),
            listeners: HashSet::new(),
            url: None
        }
    }

    async fn run(mut self) {
        loop {
            let (websocket, url) = self.connect_ws().await;

            self.url = Some(url); // Saving the url here so eventually we can implement a get_default_settings method

            self.status_tx.send(true)
                .expect("Status receiver dropped before websocket thread was closed");

            logger()
                .lock()
                .unwrap()
                .log_info("Websocket connected");

            self.main_loop(websocket).await;

            logger()
                .lock()
                .unwrap()
                .log_warn("Websocket disconnected");

            self.status_tx.send(false)
                .expect("Status receiver dropped before websocket thread was closed");
        }
    }
}

