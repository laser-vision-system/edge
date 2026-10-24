// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

use std::{
	collections::HashSet,
	env,
	fmt,
};

use api::edge::Settings;
use edge_core::logger::logger;
use pnet_datalink::interfaces;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::{
	net::TcpStream,
	task::JoinSet,
};
use tokio_tungstenite::{
	MaybeTlsStream,
	WebSocketStream,
	connect_async,
};

use crate::error::{
	EdgeClientError,
	EdgeClientResult,
};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct AddressComp {
	protocol: String,
	host: String,
	port: String,
	endpoint: String,
}

impl AddressComp {
	fn new(
		protocol: String,
		host: String,
		port: String,
		endpoint: String,
	) -> Self {
		Self {
			protocol,
			host,
			port,
			endpoint,
		}
	}

	fn local_host() -> Self {
		Self::new(
			"http://".to_owned(),
			"127.0.0.1".to_owned(),
			":8080".to_owned(),
			"/is_api".to_owned(),
		)
	}
}

impl fmt::Display for AddressComp {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(
			f,
			"{}{}{}{}",
			self.protocol, self.host, self.port, self.endpoint
		)
	}
}

fn identify_network() -> HashSet<AddressComp> {
	let mut network_addresses = interfaces()
		.into_iter()
		.filter(|network| {
			network.is_up() && !network.is_loopback() && !network.ips.is_empty()
		})
		.flat_map(|network| network.ips)
		.filter(|ip| ip.is_ipv4() && ip.prefix() >= 24)
		.flat_map(|ip| ip.iter())
		.map(|ip| {
			AddressComp::new(
				"http://".to_owned(),
				ip.to_string(),
				":8080".to_owned(),
				"/is_api".to_owned(),
			)
		})
		.collect::<HashSet<_>>();
	network_addresses.insert(AddressComp::local_host());

	network_addresses
}

struct WebSocketResponse {
	pub url: AddressComp,
	pub message: String,
}

#[expect(
	clippy::map_err_ignore,
	reason = "Temporary until error type improvements"
)]
async fn get_response(
	url: AddressComp,
	client: Client,
) -> EdgeClientResult<WebSocketResponse> {
	let message = client
		.get(url.to_string())
		.send()
		.await
		.map_err(|why| {
			EdgeClientError::ReqwestError {
				reason: why.to_string(),
			}
		})?
		.text()
		.await
		.map_err(|_| EdgeClientError::WebSocketUnpack)?;

	Ok(WebSocketResponse {
		url,
		message,
	})
}

async fn get_websocket_url(
	urls: HashSet<AddressComp>
) -> EdgeClientResult<String> {
	let client = Client::new();
	let mut set = JoinSet::new();

	for url in urls {
		set.spawn(get_response(url, client.clone()));
	}

	while let Some(result) = set.join_next().await {
		match result {
			Ok(Ok(response)) => {
				if response.message
					== "This is an API for the laser intensity analysis project"
				{
					let url = response.url;
					let new_url =
						format!("ws://{}{}/edge_ws", url.host, url.port);
					logger().log_info(&format!(
						"Correct message response recived from: {new_url}"
					));
					return Ok(new_url);
				}
			},
			#[cfg(debug_assertions)]
			Ok(Err(why)) => {
				logger().log_error(&why);
			},
			// Ignore failures silently
			_ => {},
		}
	}

	Err(EdgeClientError::AutoConnectUrl)
}

pub async fn get_ws_stream()
-> EdgeClientResult<(WebSocketStream<MaybeTlsStream<TcpStream>>, String)> {
	let websocket_url = if let Ok(url) = env::var("SERVER_URL") {
		if url.is_empty() {
			logger().log_info("Connecting using auto connect");
			let urls = identify_network();
			get_websocket_url(urls).await?
		} else {
			logger().log_info(&format!(
				"Connecting to provided server address: {url}"
			));
			format!("ws://{url}/edge_ws")
		}
	} else {
		logger().log_info("Connecting using auto connect");
		let urls = identify_network();
		get_websocket_url(urls).await?
	};

	let Ok((stream, _)) = connect_async(websocket_url.clone())
		.await
		.map_err(|why| EdgeClientError::AutoConnect(why.to_string()))
	else {
		return Err(EdgeClientError::WebSocketConnection);
	};
	Ok((stream, websocket_url))
}

// TODO: Remove this once the server request is implemented.
#[expect(
	clippy::unused_async,
	reason = "Async will be needed once this function is re-enabled"
)]
pub async fn get_default_settings(url: &str) -> EdgeClientResult<Settings> {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.

	let url = format!("http://{url}/default_config");

	let client = Client::new();
	let temp_settings = client
		.get(url)
		.send()
		.await
		.map_err(|_| EdgeClientError::ServerConnection)?;
	unpack(
		temp_settings
			.text()
			.await
			.map_err(|_| EdgeClientError::WebSocketUnpack)?
			.as_str(),
	)
}

pub fn unpack<T>(data: &str) -> EdgeClientResult<T>
where
	T: DeserializeOwned,
{
	let data_unchecked: serde_json::Result<T> = serde_json::from_str(data);
	data_unchecked.map_err(|_| EdgeClientError::WebSocketUnpack)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::client;

	#[test]
	fn chop_end() {
		let mut url = "http://".to_owned();
		url.push_str("myweb.d.0.yay");
		url.push_str(":8080/is_api");
		let mut please = url
			.rsplit('/')
			.skip(1)
			.collect::<Vec<_>>()
			.into_iter()
			.rev()
			.collect::<Vec<_>>()
			.join("");
		please.push_str(client::API);

		println!("{please}");
	}

	#[test]
	#[ignore = "IO bound"]
	fn print_interfaces() {
		interfaces()
			.into_iter()
			.filter(|network| {
				network.is_up()
					&& !network.is_loopback()
					&& !network.ips.is_empty()
			})
			.for_each(|interface| println!("{interface}"));
	}
}
