use std::io::Cursor;

use image::ImageBuffer;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::settings::{
	ENDPOINT_CAMERA_CONFIG, ENDPOINT_CAMERA_READINGS, ENDPOINT_GUI_UPDATE, ENDPOINT_IMAGE, HOST,
	PORT,
};

#[must_use]
#[derive(Serialize, Debug)]
pub struct CameraReadings {
	intensity: f32,
	deviation: f32,
}

impl CameraReadings {
	pub fn new(intensity: f32, deviation: f32) -> Self {
		Self {
			intensity,
			deviation,
		}
	}
}

#[must_use]
#[derive(Deserialize, Debug)]
pub struct GuiUpdate {
	setpoint: bool,
	config: bool,
}

impl GuiUpdate {
	pub fn setpoint(&self) -> bool {
		self.setpoint
	}

	pub fn config(&self) -> bool {
		self.config
	}
}

#[must_use]
#[derive(Deserialize, Serialize, Debug)]
pub struct CameraConfig {
	framerate: f32,
	brightness: i32,
	hue: i32,
	saturation: i32,
}

impl CameraConfig {
	pub fn new(framerate: f32, brightness: i32, hue: i32, saturation: i32) -> Self {
		Self {
			framerate,
			brightness,
			hue,
			saturation,
		}
	}

	pub fn framerate(&self) -> f32 {
		self.framerate
	}

	pub fn brightness(&self) -> i32 {
		self.brightness
	}

	pub fn hue(&self) -> i32 {
		self.hue
	}

	pub fn saturation(&self) -> i32 {
		self.saturation
	}
}

// TODO: Look into making this a singleton or global.
#[must_use]
pub struct Client(reqwest::Client);

impl Default for Client {
	fn default() -> Self {
		Self::new()
	}
}

impl Client {
	pub fn new() -> Self {
		Self(reqwest::Client::new())
	}

	fn default_address() -> String {
		format!("http://{HOST}:{PORT}")
	}

	fn url_from_endpoint(endpoint: &str) -> reqwest::Url {
		let url = format!("{}/{}", Self::default_address(), endpoint);
		reqwest::Url::parse(&url).unwrap()
	}

	/// Post camera info: `/camera_info`
	pub async fn post_readings(&self, data: CameraReadings) -> Result<(), reqwest::Error> {
		let url = Self::url_from_endpoint(ENDPOINT_CAMERA_READINGS);
		self.0
			.post(url)
			.json(&data)
			.send()
			.await?
			.error_for_status()?;
		Ok(())
	}

	/// Post configuration: `/configuration`
	pub async fn post_camera_config(&self, data: CameraConfig) -> Result<(), reqwest::Error> {
		let url = Self::url_from_endpoint(ENDPOINT_CAMERA_CONFIG);
		self.0
			.post(url)
			.json(&data)
			.send()
			.await?
			.error_for_status()?;
		Ok(())
	}

	/// Get configuration: `/configuration`
	pub async fn get_camera_config(&self) -> Result<CameraConfig, reqwest::Error> {
		let url = Self::url_from_endpoint(ENDPOINT_CAMERA_CONFIG);
		self.0.get(url).send().await?.json::<CameraConfig>().await
	}

	/// Get GUI update: `/gui_update`
	pub async fn get_gui_update(&self) -> Result<GuiUpdate, reqwest::Error> {
		let url = Self::url_from_endpoint(ENDPOINT_GUI_UPDATE);
		self.0.get(url).send().await?.json::<GuiUpdate>().await
	}

	/// Put image: `/image`
	pub async fn put_image(
		&self,
		data: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
	) -> Result<(), reqwest::Error> {
		let mut buff = Cursor::new(Vec::new());
		let mut enc = image::codecs::jpeg::JpegEncoder::new(&mut buff);
		enc.encode_image(&data).unwrap();

		let part = reqwest::multipart::Part::bytes(buff.into_inner());
		let multipart = reqwest::multipart::Form::new().part("file", part);

		let url = Self::url_from_endpoint(ENDPOINT_IMAGE);
		self.0
			.put(url)
			.multipart(multipart)
			.send()
			.await?
			.error_for_status()?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::process::Termination;

	use super::*;

	#[tokio::test]
	async fn post_readings() -> Result<(), reqwest::Error> {
		let client = Client::new();
		let reading = CameraReadings {
			intensity: 72.2,
			deviation: 0.21,
		};
		client.post_readings(reading).await
	}

	#[tokio::test]
	async fn post_camera_config() -> Result<(), reqwest::Error> {
		let client = Client::new();
		let config = CameraConfig {
			framerate: 30.0,
			brightness: 100,
			hue: 0,
			saturation: 100,
		};
		client.post_camera_config(config).await
	}

	// A test returning a `CameraConfig` always counts as a success.
	impl Termination for CameraConfig {
		fn report(self) -> std::process::ExitCode {
			std::process::ExitCode::SUCCESS
		}
	}

	#[tokio::test]
	async fn get_camera_config() -> Result<CameraConfig, reqwest::Error> {
		let client = Client::new();
		client.get_camera_config().await
	}

	// A test returning a `GuiUpdate` always counts as a success.
	impl Termination for GuiUpdate {
		fn report(self) -> std::process::ExitCode {
			std::process::ExitCode::SUCCESS
		}
	}

	#[tokio::test]
	async fn get_gui_update() -> Result<GuiUpdate, reqwest::Error> {
		let client = Client::new();
		client.get_gui_update().await
	}

	#[tokio::test]
	async fn put_image() -> Result<(), reqwest::Error> {
		let client = Client::new();
		let image = ImageBuffer::<image::Rgb<u8>, Vec<_>>::new(50, 50);
		client.put_image(image).await
	}
}
