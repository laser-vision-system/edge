use std::time::Duration;

use futures_util::{SinkExt, StreamExt, stream::FusedStream};
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

pub type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct WebSocket {
	inner: WebsocketType,
}

impl WebSocket {
	pub fn new(inner: WebsocketType) -> Self {
		Self { inner }
	}

	pub async fn websocket_connect(url: &str) -> Self {
		let (stream, _) = connect_async(url).await.expect("Failed to connect");
		Self { inner: stream }
	}

	pub fn is_terminated(&self) -> bool {
		self.inner.is_terminated()
	}

	pub async fn websocket_connect_static() -> Self {
		let url = "ws://".to_string() + HOST + ":" + PORT + API;
		Self::websocket_connect(&url).await
	}

	pub async fn keep_alive(
		&mut self,
	) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
		let msg = "hello";
		self.inner.send(Message::Pong(Bytes::from(msg))).await?;
		Ok(())
	}

	pub async fn send_text<T: Serialize>(
		&mut self,
		mut prefix: String,
		data: T,
	) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
		prefix.push_str(&*serde_json::to_string(&data)?);
		self.inner
			.send(Message::Text(Utf8Bytes::from(prefix)))
			.await?;
		Ok(())
	}

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
						logger().lock().unwrap().log_info(&format!("Received message: {}", text));
						process_incoming(&text).await
					}
					_ => None,
				}
			}
			_ = interval.tick() => None
		}
	}
}

async fn process_incoming(text: &str) -> Option<EdgeRequests> {
	let msg = text.trim();

	println!("Received from Server: {}", msg);

	let mut args = msg.splitn(2, ':');

	match args.next().unwrap() {
		"SetNameCamera" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::SetNameCamera(inner))
				})
			}),
		},
		"StartViewingCamera" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::StartViewingCamera(inner))
				})
			}),
		},
		"StopViewingCamera" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::StopViewingCamera(inner))
				})
			}),
		},
		"Configuration" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::Configuration(inner))
				})
			}),
		},
		"ViewCameraConfig" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::ViewCameraConfig(inner))
				})
			}),
		},
		"Setpoint" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked
					.map_or(None, |inner| Some(EdgeRequests::Setpoint(inner)))
			}),
		},
		"SetProcessingType" => match args.next() {
			data => data.map_or(None, |inner| {
				let unpacked = unpack(inner);
				unpacked.map_or(None, |inner| {
					Some(EdgeRequests::SetProcessingType(inner))
				})
			}),
		},
		_ => None,
	}
}
