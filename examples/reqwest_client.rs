//use image::ImageBuffer;
use serde::{Deserialize, Serialize};

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8080";

const ENDPOINT_CAMERA_READINGS: &str = "/camera_info";
const ENDPOINT_CAMERA_CONFIG: &str = "/configuration";
const ENDPOINT_GUI_UPDATE: &str = "/gui_update";
//const ENDPOINT_IMAGE: &str = "/image";

#[derive(Deserialize, Serialize, Debug)]
struct CameraReadings {
	intensity: f32,
	deviation: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct GUIUpdate {
	setpoint: bool,
	config: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct CameraConfig {
	framerate: f32,
	brightness: i32,
	hue: i32,
	saturation: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let host = format!("http://{ADDRESS}:{PORT}");

	let client = reqwest::Client::new();

	let reading = CameraReadings {
		intensity: 72.2,
		deviation: 0.21,
	};
	let url = host.clone() + ENDPOINT_CAMERA_READINGS;
	post_readings(&client, &url, reading).await;

	let config = CameraConfig {
		framerate: 30.0,
		brightness: 100,
		hue: 0,
		saturation: 100,
	};
	let url = host.clone() + ENDPOINT_CAMERA_CONFIG;
	post_camera_config(&client, &url, config).await;

	let url = host.clone() + ENDPOINT_CAMERA_CONFIG;
	let new_config = get_camera_config(&client, &url).await;
	println!("{new_config:#?}");

	let url = host.clone() + ENDPOINT_GUI_UPDATE;
	let gui_update = get_gui_update(&client, &url).await;
	println!("{gui_update:#?}");

	//let image = ImageBuffer::<image::Rgb<u8>, Vec<_>>::new(10, 10);
	//let url = host.clone() + ENDPOINT_IMAGE;
	//put_image(&client, &url, image).await;

	Ok(())
}

/// Post camera info
/// `/camera_info`
async fn post_readings(client: &reqwest::Client, url: &str, data: CameraReadings) {
	client
		.post(url)
		.json(&data)
		.send()
		.await
		.unwrap()
		.error_for_status()
		.unwrap();
}

/// Post configuration
/// `/configuration`
async fn post_camera_config(client: &reqwest::Client, url: &str, data: CameraConfig) {
	client
		.post(url)
		.json(&data)
		.send()
		.await
		.unwrap()
		.error_for_status()
		.unwrap();
}

/// Get configuration
/// `/configuration`
async fn get_camera_config(client: &reqwest::Client, url: &str) -> CameraConfig {
	client
		.get(url)
		.send()
		.await
		.unwrap()
		.json::<CameraConfig>()
		.await
		.unwrap()
}

/// Get GUI update
/// `/gui_update`
async fn get_gui_update(client: &reqwest::Client, url: &str) -> GUIUpdate {
	client
		.get(url)
		.send()
		.await
		.unwrap()
		.json::<GUIUpdate>()
		.await
		.unwrap()
}

///// Put image
///// `/image`
//async fn put_image(
//	client: &reqwest::Client,
//	url: &str,
//	data: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
//) {
//	let multipart = reqwest::multipart::Form::new();
//	let idk = data.clone();
//	let part = reqwest::multipart::Part::bytes(idk.to_byte_slice());
//	let multipart = multipart.part("file", part);
//	client
//		.put(url)
//		.multipart(multipart)
//		.send()
//		.await
//		.unwrap()
//		.error_for_status()
//		.unwrap();
//}
