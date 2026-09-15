use std::fmt;

use crate::logger::logger;
use crate::{
	error::{EdgeError, Result},
	websocket::WebSocket,
};
use pnet_datalink::interfaces;
use reqwest::Client;
use server_edge_datatypes::edge::Settings;
use tokio::task::JoinSet;
use tokio_tungstenite::connect_async;

#[derive(Clone, Debug, Default)]
struct AddressComp {
	protocol: String,
	host: String,
	port: String,
	endpoint: String,
}

impl AddressComp {
	fn new(protocol: String, host: String, port: String, endpoint: String) -> Self {
		Self {
			protocol,
			host,
			port,
			endpoint,
		}
	}
	fn local_host() -> Self {
		Self::new(
			"http://".to_string(),
			"127.0.0.1".to_string(),
			":8080".to_string(),
			"/is_api".to_string(),
		)
	}
}

impl fmt::Display for AddressComp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}{}{}",
			self.protocol, self.host, self.port, self.endpoint
		)
	}
}

async fn identify_network() -> Result<Vec<AddressComp>> {
	let mut network_addresses = interfaces()
		.into_iter()
		.filter(|network| network.is_up() && !network.is_loopback() && !network.ips.is_empty())
		.flat_map(|network| network.ips)
		.filter(|ip| ip.is_ipv4())
		.flat_map(|ip| ip.iter())
		.map(|ip| {
			AddressComp::new(
				"http://".to_string(),
				ip.to_string(),
				":8080".to_string(),
				"/is_api".to_string(),
			)
		})
		.collect::<Vec<_>>();

	network_addresses.push(AddressComp::local_host());

	if network_addresses.len() > 0 {
		Ok(network_addresses.clone())
	} else {
		Err(EdgeError::IdentifyNetwork)
	}
}

struct WebSocketResponse {
	pub url: AddressComp,
	pub message: String,
}

async fn get_response(url: AddressComp, client: Client) -> Result<WebSocketResponse> {
	let message = client
		.get(&url.to_string())
		.send()
		.await
		.map_err(|why| EdgeError::ReqwestError {
			reason: why.to_string(),
		})?
		.text()
		.await
		.map_err(|_| EdgeError::WebSocketUnpack)?;

	Ok(WebSocketResponse { url, message })
}

async fn get_websocket_url(urls: Vec<AddressComp>) -> Result<String> {
	let client = Client::new();
	let mut set = JoinSet::new();

	for url in urls {
		set.spawn(get_response(url, client.clone()));
	}

	while let Some(result) = set.join_next().await {
		match result {
			Ok(Ok(response)) => {
				if response.message == "This is an API for the laser intensity analysis project" {
					let url = response.url;
					let new_url = format!("ws://{}{}/edge_ws", url.host, url.port);
					return Ok(new_url);
				}
			},
			#[cfg(debug_assertions)]
			Ok(Err(why)) => {
				logger()
					.lock()
					.unwrap()
					.log_error(&format!("Incorrect message response"), &why);
			},
			// Ignore failures silently
			_ => {},
		}
	}

	Err(EdgeError::AutoConnect)
}

/// When call,
pub async fn assert_connection(
	websocket: Option<WebSocket>,
	settings: Option<Settings>,
) -> (WebSocket, Settings) {
	match (websocket, settings) {
		// All good, normal operation.
		(Some(socket), Some(settings)) => (socket, settings),
		// Lost connection to server
		(None, _) => loop_until_connection().await,
		// Fallback
		_ => loop_until_connection().await,
	}
}

async fn loop_until_connection() -> (WebSocket, Settings) {
	// TODO: Loop here, with delay.
	match try_connect_once().await {
		Ok(yay) => yay,
		Err(err) => {
			logger().lock().unwrap().log_error("PLEASE", &err);
			panic!("SERVER FAILED TO CONNECT")
		},
	}
}

async fn try_connect_once() -> Result<(WebSocket, Settings)> {
	let urls = identify_network().await?;
	let websocket_url = get_websocket_url(urls).await?;
	let (stream, _) =
		connect_async(websocket_url.clone())
			.await
			.map_err(|why| EdgeError::AutoConnectUrl {
				reason: why.to_string(),
			})?;
	let return_websocket = WebSocket::new(stream);
	let return_settings = get_default_settings(&websocket_url.to_string()).await?;
	Ok((return_websocket, return_settings))
}

async fn get_default_settings(url: &str) -> Result<Settings> {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.

	Ok(Settings::default())

	//let client = Client::new();
	//let temp_settings = client
	//	.get(url)
	//	.send()
	//	.await
	//	.map_err(|_| EdgeError::ServerConnection)?;
	//unpack(
	//	temp_settings
	//		.text()
	//		.await
	//		.map_err(|_| EdgeError::WebSocketUnpack)?
	//		.as_str(),
	//)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::settings;

	#[test]
	fn chop_end() {
		let mut url = "http://".to_string();
		url.push_str("myweb.d.0.yay");
		url.push_str(":8080/is_api");
		let mut please = url
			.rsplit("/")
			.skip(1)
			.collect::<Vec<_>>()
			.into_iter()
			.rev()
			.collect::<Vec<_>>()
			.join("");
		please.push_str(settings::API);

		println!("{please}");
	}

	#[test]
	fn print_interfaces() {
		interfaces()
			.into_iter()
			.filter(|network| network.is_up() && !network.is_loopback() && !network.ips.is_empty())
			.for_each(|interface| println!("{}", interface));
	}
}
