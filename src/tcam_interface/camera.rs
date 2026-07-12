//! # Camera
//!
//! An abstraction over the nokhwa camera objects.
//!
//! TODO: Either use a callback camera in all cases, or dynamically swap between a polling and callback camera.

use image::RgbImage;
use nokhwa;

enum CamType {
	Polling(nokhwa::Camera),
	Callback(nokhwa::CallbackCamera),
}

impl CamType {
	pub fn get_frame(&mut self) -> Result<RgbImage, nokhwa::NokhwaError> {
		match self {
			Self::Polling(camera) => {
				let frame = camera.frame()?;
				let decoded: RgbImage = frame.decode_image::<nokhwa::pixel_format::RgbFormat>()?;
				Ok(decoded)
			},
			Self::Callback(camera) => {
				eprintln!("Avoid calling `get_frame` on a callback camera");
				let frame = camera.poll_frame()?;
				let decoded: RgbImage = frame.decode_image::<nokhwa::pixel_format::RgbFormat>()?;
				Ok(decoded)
			},
		}
	}
}

pub struct Camera(CamType);

impl Camera {
	pub fn new_polling(index: u32) -> Result<Self, nokhwa::NokhwaError> {
		let format = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
			nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate,
		);

		// TODO Handle potential error cases.
		let mut camera = nokhwa::Camera::new(nokhwa::utils::CameraIndex::Index(index), format)?;
		camera.open_stream()?;

		Ok(Camera(CamType::Polling(camera)))
	}

	pub fn new_callback(
		index: u32,
		mut callback: impl FnMut(RgbImage) + Send + 'static,
	) -> Result<Self, nokhwa::NokhwaError> {
		let format = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(
			nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate,
		);

		let internal_callback = move |buffer: nokhwa::Buffer| {
			let frame: RgbImage = buffer
				.decode_image::<nokhwa::pixel_format::RgbFormat>()
				.unwrap();
			callback(frame);
		};

		// TODO Handle potential error cases.
		let mut camera = nokhwa::CallbackCamera::new(
			nokhwa::utils::CameraIndex::Index(index),
			format,
			internal_callback,
		)?;
		camera.open_stream()?;

		Ok(Camera(CamType::Callback(camera)))
	}

	pub fn get_frame(&mut self) -> Result<RgbImage, nokhwa::NokhwaError> {
		self.0.get_frame()
	}

	// reconnect

	// report configurations

	// apply configurations
}
