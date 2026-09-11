use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::{self, net::TcpStream, time::Interval};
use tokio_tungstenite::{
	MaybeTlsStream, WebSocketStream, connect_async,
	tungstenite::{Bytes, Message, Utf8Bytes},
};

use crate::{
	error::{EdgeError, Result},
	settings::{API, HOST, PORT},
};

pub type WebsocketType = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct WebSocket {
	inner: WebsocketType,
}

impl WebSocket {
	pub async fn websocket_connect(url: &str) -> Self {
		let (stream, _) = connect_async(url).await.expect("Failed to connect");
		Self { inner: stream }
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

	pub async fn query_websockets(&mut self, mut interval: Interval) -> bool {
		tokio::select! {
			Some(Ok(msg)) = self.inner.next() => {
				match msg {
					Message::Text(text) => {
						println!("{}", text );
						// TODO
						//self.client.websocket_incoming_data(&text).await;
						true
					}
					_ => {true},
				}
			}
			_ = interval.tick() => false
		}
	}
}

pub fn unpack<T: DeserializeOwned>(data: &str) -> Result<T> {
	let data_unchecked: serde_json::Result<T> = serde_json::from_str(data);
	data_unchecked.map_err(|_| EdgeError::WebSocketUnpack)
}
