use crate::network::unpack;
use crate::{
	error::{EdgeError, Result},
	websocket::WebSocket,
};
use pnet_datalink::interfaces;
use reqwest::Client;
use server_edge_datatypes::edge::Settings;
use tokio::task::JoinSet;
use tokio_tungstenite::connect_async;

pub async fn identify_network() -> std::result::Result<Vec<String>, String> {
	let all_interfaces = interfaces();
	let default_interface = all_interfaces
		.iter()
		.find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

	match default_interface {
		Some(interface) => {
			let mut network_addresses = Vec::new();
			for networks in interface.ips.iter() {
				if networks.is_ipv4() == true {
					for raw_address in networks.iter() {
						if raw_address.is_ipv4() == true {
							let mut url = "http://".to_string();
							url.push_str(raw_address.to_string().as_str());
							url.push_str(":8080/is_api");
							network_addresses.push(url);
						}
					}
				}
			}
			Ok(network_addresses)
		},
		None => {
			println!("Error while finding the default interface.");
			Err("Error while finding the default interface.".to_string())
		},
	}
}

pub async fn websocket_connect_automatic()
-> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
	let client = Client::new();
	let urls: Vec<String> = identify_network().await?;

	let mut set = JoinSet::new();

	for url in urls {
		set.spawn(get_response(url, client.clone()));
	}

	async fn get_response(
		url: String,
		client: Client,
	) -> std::result::Result<(String, String), Box<reqwest::Error>> {
		let result = client.get(&url[..]).send().await;
		match result {
			Ok(a) => Ok((url, a.text().await?)),
			Err(e) => Err(Box::new(e)),
		}
	}

	while let Some(result) = set.join_next().await {
		match result {
			Ok(Ok(a)) => {
				if a.1 == "This is an API for the laser intensity analysis project" {
					return Ok(a.0.clone());
				}
			},
			_ => {},
		}
	}

	Err(Box::new(EdgeError::AutoConnect))
}

/// When call,
pub async fn connect_loop(
	websocket: Option<WebSocket>,
	settings: Option<Settings>,
) -> Result<(WebSocket, Settings)> {
	match (websocket, settings) {
		// All good, normal operation.
		(Some(socket), Some(settings)) => Ok((socket, settings)),
		// Lost connection to server
		(None, _) => new_connection().await,
		// Fallback
		_ => new_connection().await,
	}
}

async fn new_connection() -> Result<(WebSocket, Settings)> {
	let url = websocket_connect_automatic()
		.await
		.map_err(|_| EdgeError::AutoConnect)?;
	let (stream, _) = connect_async(url.clone())
		.await
		.map_err(|_| EdgeError::AutoConnect)?;
	let return_websocket = WebSocket::new(stream);
	let return_settings = get_default_settings(&url)
		.await
		.map_err(|_| EdgeError::AutoConnect)?;
	Ok((return_websocket, return_settings))
}

async fn get_default_settings(url: &str) -> Result<Settings> {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.
	let client = Client::new();
	let temp_settings = client
		.get(url)
		.send()
		.await
		.map_err(|_| EdgeError::ServerConnection)?;
	unpack(
		temp_settings
			.text()
			.await
			.map_err(|_| EdgeError::WebSocketUnpack)?
			.as_str(),
	)
}
