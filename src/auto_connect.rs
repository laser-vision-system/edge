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
	let host_interfaces = interfaces();
	let host_active_interfaces = host_interfaces.iter().filter(|network| !network.ips.is_empty() && network.is_up() == true).collect::<Vec<_>>();
	let mut network_addresses = Vec::new();
	for interface in host_active_interfaces {
		if interface.is_loopback() == true {
			network_addresses.push(AddressComp::new(
				"http://".to_string(),
				"127.0.0.1".to_string(),
				":8080".to_string(),
				"/is_api".to_string(),));
			continue
		}
		if !interface.ips.is_empty() && interface.is_up() == true {
			let _interface = interface.ips.iter()
				.filter(|ip| ip.is_ipv4() == true)
				.map(|ip| network_addresses.push(AddressComp::new(
					"http://".to_string(),
					"127.0.0.1".to_string(),
					":8080".to_string(),
					"/is_api".to_string(),)));
		}
	}
	if network_addresses.len() > 0 {
		Ok(network_addresses.clone())
	}
	else {
		Err(EdgeError::IdentifyNetwork)
	}
}


pub async fn websocket_connect_automatic() -> Result<String> {
	let client = Client::new();
	let urls: Vec<_> = identify_network().await?;

	let mut set = JoinSet::new();

	for url in urls {
		set.spawn(get_response(url, client.clone()));
	}

	async fn get_response(url: AddressComp, client: Client) -> Result<(AddressComp, String)> {
		let result = client.get(&url.to_string()).send().await;
		match result {
			Ok(a) => Ok((url, a.text().await.map_err(|_| EdgeError::WebSocketUnpack)?)),
			Err(e) => Err(EdgeError::ReqwestError {
				reason: e.to_string(),
			}),
		}
	}

	while let Some(result) = set.join_next().await {
		match result {
			Ok(Ok(a)) => {
				if a.1 == "This is an API for the laser intensity analysis project" {
					let idk = a.0;
					let new_url = format!("ws://{}{}/edge_ws", idk.host, idk.port);
					return Ok(new_url);
				}
			},
			_ => {},
		}
	}

	Err(EdgeError::AutoConnect)
}

/// When call,
pub async fn connect_loop(
	websocket: Option<WebSocket>,
	settings: Option<Settings>,
) -> (WebSocket, Settings) {
	match (websocket, settings) {
		// All good, normal operation.
		(Some(socket), Some(settings)) => (socket, settings),
		// Lost connection to server
		(None, _) => new_connection().await,
		// Fallback
		_ => new_connection().await,
	}
}

async fn new_connection() -> (WebSocket, Settings) {
	match try_connect_once().await {
		Ok(yay) => yay,
		Err(err) => {
			logger().lock().unwrap().log_error("PLEASE", &err);
			panic!("SERVER FAILED TO CONNECT")
		},
	}
}

async fn try_connect_once() -> Result<(WebSocket, Settings)> {
	let url = websocket_connect_automatic().await?;
	let (stream, _) =
		connect_async(url.clone())
			.await
			.map_err(|why| EdgeError::AutoConnectUrl {
				reason: why.to_string(),
			})?;
	let return_websocket = WebSocket::new(stream);
	let return_settings = get_default_settings(&url.to_string()).await?;
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
}
