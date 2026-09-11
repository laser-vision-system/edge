use futures_util::stream::FusedStream;
use pnet_datalink::interfaces;
use reqwest::Client;
use server_edge_datatypes::edge::Settings;
use tokio::net::TcpStream;
use tokio::task::JoinSet;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::{error::EdgeError, websocket::WebSocket};

pub async fn identify_network() -> Result<Vec<String>, String> {
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
-> Result<String, Box<dyn std::error::Error + Send + Sync>> {
	let client = Client::new();
	let urls: Vec<String> = identify_network().await?;

	let mut set = JoinSet::new();

	for url in urls {
		set.spawn(get_response(url, client.clone()));
	}

	async fn get_response(
		url: String,
		client: Client,
	) -> Result<(String, String), Box<reqwest::Error>> {
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
	mut websocket: Option<WebSocket>,
	settings: Option<Settings>,
) -> Result<(WebSocket, Settings), Box<dyn std::error::Error + Send + Sync>> {
	let connection_established = websocket.is_none();
	if !connection_established {
		let client = Client::new();
		let url = websocket_connect_automatic().await?;
		let (return_websocket, response) = connect_async(url).await?;
		let temp_settings  = client.get(&url[..]).send().await?;
		let return_settings =
		return Ok((Some(return_websocket), Settings::default()))
	}

	Ok((Some(websocket), Settings::default()))

}

fn get_default_settings(settings: Option<Settings>) -> Settings {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.
	Settings::default()
}
