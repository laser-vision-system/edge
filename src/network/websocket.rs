// TODO: Remove all unwraps and then this.
#![expect(
	clippy::unwrap_used,
	clippy::unwrap_in_result,
	reason = "WIP development"
)]

use std::time::Duration;

use futures_util::{SinkExt as _, StreamExt as _, stream::FusedStream as _};
use serde::Serialize;
use server_edge_datatypes::edge::EdgeRequests;
use tokio::{self, net::TcpStream, time::interval};
use tokio_tungstenite::{
	MaybeTlsStream, WebSocketStream, connect_async,
	tungstenite::{Bytes, Message, Utf8Bytes},
};

use crate::{
	logger::logger,
	network::unpack,
	settings::{API, HOST, PORT},
};

type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct WebSocket {
	inner: WebsocketType,
}

impl WebSocket {
	pub fn new(inner: WebsocketType) -> Self {
		Self { inner }
	}

	#[expect(
		dead_code,
		reason = "Needs to be better integrated with autonnect"
	)]
	pub async fn websocket_connect(url: &str) -> Self {
		let (stream, _) = connect_async(url).await.expect("Failed to connect");
		Self { inner: stream }
	}

	pub fn is_terminated(&self) -> bool {
		self.inner.is_terminated()
	}

	#[expect(
		dead_code,
		reason = "Needs to be better integrated with autonnect"
	)]
	pub async fn websocket_connect_static() -> Self {
		let url = "ws://".to_owned() + HOST + ":" + PORT + API;
		Self::websocket_connect(&url).await
	}

	pub async fn keep_alive(
		&mut self,
	) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let msg = "hello";
		self.inner.send(Message::Pong(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_text<T>(
		&mut self,
		data: T,
	) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>
	where
		T: Serialize,
	{
		self.inner
			.send(Message::Text(Utf8Bytes::from(&serde_json::to_string(&data)?)))
			.await?;
		Ok(())
	}

	#[expect(
		clippy::big_endian_bytes,
		reason = "This data is intended to be sent to the server"
	)]
	pub async fn send_image(
		&mut self,
		camera_id: u64,
		frame: &mut Vec<u8>,
	) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let mut msg = camera_id.to_be_bytes().to_vec();
		msg.append(frame);
		self.inner.send(Message::Binary(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn query_websockets(
		&mut self,
		period: Duration,
	) -> Option<EdgeRequests> {
		let mut interval = interval(period);
		tokio::select! {
			Some(Ok(msg)) = self.inner.next() => {
				match msg {
					Message::Text(text) => {
						#[cfg(debug_assertions)]
						logger().lock().unwrap().log_info(&format!("Received message: {text}"));
						process_incoming(&text)
					}
					_ => None,
				}
			}
			_ = interval.tick() => None
		}
	}
}

fn process_incoming(text: &str) -> Option<EdgeRequests> {
	let msg_unchecked: Result<EdgeRequests, _> = serde_json::from_str(text.trim());

	logger()
		.lock()
		.unwrap()
		.log_info(&format!("Received from Server: {}", text.trim()));

	let Ok(msg) = msg_unchecked else {
		return None
	};

	match msg {
		EdgeRequests::SetNameCamera(_) => Some(msg),
		EdgeRequests::StartViewingCamera(_) => Some(msg),
		EdgeRequests::StopViewingCamera(_) => Some(msg),
		EdgeRequests::Configuration(_) => {
			println!("Hello you matched me properly!!!!");
			Some(msg)
		},
		EdgeRequests::ViewCameraConfig(_) => Some(msg),
		EdgeRequests::Setpoint(_) => Some(msg),
		EdgeRequests::SetProcessingType(_) => Some(msg),
		_ => None,
	}

	// match args.next().unwrap() {
	// 	"SetNameCamera" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked
	// 			.map_or(None, |inner| Some(EdgeRequests::SetNameCamera(inner)))
	// 	},
	// 	"StartViewingCamera" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked.map_or(None, |inner| {
	// 			Some(EdgeRequests::StartViewingCamera(inner))
	// 		})
	// 	},
	// 	"StopViewingCamera" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked.map_or(None, |inner| {
	// 			Some(EdgeRequests::StopViewingCamera(inner))
	// 		})
	// 	},
	// 	"Configuration" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked
	// 			.map_or(None, |inner| Some(EdgeRequests::Configuration(inner)))
	// 	},
	// 	"ViewCameraConfig" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked.map_or(None, |inner| {
	// 			Some(EdgeRequests::ViewCameraConfig(inner))
	// 		})
	// 	},
	// 	"Setpoint" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked.map_or(None, |inner| Some(EdgeRequests::Setpoint(inner)))
	// 	},
	// 	"SetProcessingType" => {
	// 		let data = args.next();
	// 		let inner = data?;
	// 		let unpacked = unpack(inner);
	// 		unpacked.map_or(None, |inner| {
	// 			Some(EdgeRequests::SetProcessingType(inner))
	// 		})
	// 	},
	// 	_ => None,
	// }
}
