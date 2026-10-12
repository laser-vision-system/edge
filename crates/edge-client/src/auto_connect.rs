// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

use std::{
	collections::HashSet,
	env,
	fmt,
};

use api::edge::{
	InitialiseConnection,
	ServerRequests,
	Settings,
};
use edge_cameras::persistence::Persistence;
use edge_core::{
	logger::logger,
	settings::{
		AUTO_CONNECT_DELAY,
		EnvVariables,
	},
};
use pnet_datalink::interfaces;
use reqwest::Client;
use tokio::{
	task::JoinSet,
	time::interval,
};
use tokio_tungstenite::connect_async;

use crate::{
	error::{
		EdgeClientError,
		EdgeClientResult,
	},
	persistent_edge_id::get_mac_address,
	websocket::WebSocket,
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
					logger().lock().unwrap().log_info(&format!(
						"Correct message response recived from: {new_url}"
					));
					return Ok(new_url);
				}
			},
			#[cfg(debug_assertions)]
			Ok(Err(why)) => {
				logger().lock().unwrap().log_error(&why);
			},
			// Ignore failures silently
			_ => {},
		}
	}

	Err(EdgeClientError::AutoConnect)
}

pub async fn send_persistent_edge_id(mut websocket: WebSocket) -> WebSocket {
	let request_enum =
		ServerRequests::InitialiseConnection(InitialiseConnection {
			edge_id: get_mac_address().unwrap(),
		});
	_ = websocket.send_text(request_enum).await;
	websocket
}

pub async fn send_camera_list(
	persistence: &mut Persistence,
	mut websocket: WebSocket,
) -> WebSocket {
	_ = websocket.send_text(persistence.get_current_cameras()).await;
	websocket
}

pub async fn assert_connection(
	websocket: Option<WebSocket>,
	settings: Option<Settings>,
	persistence: &mut Persistence,
	env_variables: &EnvVariables,
) -> (WebSocket, Settings) {
	// All good, normal operation.
	if let (Some(socket), Some(settings)) = (websocket, settings) {
		if socket.is_terminated() {
			let (socket, settings) = loop_until_connection(env_variables).await;
			let socket = send_persistent_edge_id(socket).await;
			(send_camera_list(persistence, socket).await, settings)
		} else {
			(socket, settings)
		}
	}
	// First time calling.
	else {
		let (socket, settings) = loop_until_connection(env_variables).await;
		(send_persistent_edge_id(socket).await, settings)
	}
}

async fn loop_until_connection(
	env_variables: &EnvVariables
) -> (WebSocket, Settings) {
	let mut limit_interval = interval(AUTO_CONNECT_DELAY);
	loop {
		match try_connect_once(env_variables).await {
			Ok(yay) => {
				logger()
					.lock()
					.unwrap()
					.log_info("Successfully connected to server!");
				return yay;
			},
			Err(err) => {
				logger().lock().unwrap().log_error(&err);
			},
		}
		limit_interval.tick().await;
	}
}

async fn try_connect_once(
	_env_variables: &EnvVariables
) -> EdgeClientResult<(WebSocket, Settings)> {
	let websocket_url = if let Ok(url) = env::var("SERVER_URL") {
		if url.is_empty() {
			logger()
				.lock()
				.unwrap()
				.log_info("Connecting using auto connect");
			let urls = identify_network();
			get_websocket_url(urls).await?
		} else {
			logger().lock().unwrap().log_info(&format!(
				"Connecting to provided server address: {url}"
			));
			format!("ws://{url}/edge_ws")
		}
	} else {
		logger()
			.lock()
			.unwrap()
			.log_info("Connecting using auto connect");
		let urls = identify_network();
		get_websocket_url(urls).await?
	};

	let (stream, _) =
		connect_async(websocket_url.clone()).await.map_err(|why| {
			EdgeClientError::AutoConnectUrl {
				reason: why.to_string(),
			}
		})?;
	let return_websocket = WebSocket::new(stream);
	let return_settings = get_default_settings(&websocket_url.clone()).await?;
	Ok((return_websocket, return_settings))
}

// TODO: Remove this once the server request is implemented.
#[expect(
	clippy::unused_async,
	reason = "Async will be needed once this function is re-enabled"
)]
async fn get_default_settings(_url: &str) -> EdgeClientResult<Settings> {
	// TODO: Get the default settings from the server
	// Build the settings struct from the received data.
	// If the server fails to send correct settings, use Settings::default() as
	// a fallback and report the issue to the server in as much detail as
	// possible.

	Ok(Settings::default())

	// let client = Client::new();
	// let temp_settings = client
	// 	.get(url)
	// 	.send()
	// 	.await
	// 	.map_err(|_| EdgeClientError::ServerConnection)?;
	// unpack(
	// 	temp_settings
	// 		.text()
	// 		.await
	// 		.map_err(|_| EdgeClientError::WebSocketUnpack)?
	// 		.as_str(),
	//)
}

#[cfg(test)]
mod tests {
	use edge_core::settings;

	use super::*;

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
		please.push_str(settings::API);

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
