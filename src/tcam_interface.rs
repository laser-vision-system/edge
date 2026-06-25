//! # Tcam Interface
//! Provides connection and configuration for connected cameras.
//!
//! ## Intended functionality
//! - On start-up, identify all connected cameras.
//! - Detect when a new camera is connected.
//! - Detect when a camera disconnects.
//! - Create a connection to a camera.
//! - Read input from the camera.
//! - Provide details about connected camera configurations.
//! - Apply provided configurations.
//!
//! ## Expected modules
//! ### Camera
//! A module containing a representational object for a single camera.
//!
//! ### Configurations
//! A module containing configuration options available to the camera.
//!
//! Perhaps separated by the what types of cameras they can be applied to.
//!
//! ### Detection
//! A module containing functions to detect newly connected cameras.
//!
//! These can be called at startup.

use image::RgbImage;
use nokhwa::{
	self, Buffer, CallbackCamera, Camera, NokhwaError,
	pixel_format::RgbFormat,
	utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

// TODO Look into making internals private.
pub enum TcamInterface {
	Polling { camera: Camera },
	Callback { camera: CallbackCamera },
}

impl TcamInterface {
	pub fn new_polling(index: u32) -> Result<Self, NokhwaError> {
		let format =
			RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

		// TODO Handle potential error cases.
		let mut camera = Camera::new(CameraIndex::Index(index), format)?;
		camera.open_stream()?;

		Ok(TcamInterface::Polling { camera })
	}

	pub fn new_callback(
		index: u32,
		mut callback: impl FnMut(RgbImage) + Send + 'static,
	) -> Result<Self, NokhwaError> {
		let format =
			RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

		let internal_callback = move |buffer: Buffer| {
			let frame: RgbImage = buffer.decode_image::<RgbFormat>().unwrap();
			callback(frame);
		};

		// TODO Handle potential error cases.
		let mut camera = CallbackCamera::new(CameraIndex::Index(index), format, internal_callback)?;
		camera.open_stream()?;

		Ok(TcamInterface::Callback { camera })
	}

	// reconnect

	pub fn get_frame(&mut self) -> Result<RgbImage, NokhwaError> {
		match self {
			Self::Polling { camera } => {
				let frame = camera.frame()?;
				let decoded: RgbImage = frame.decode_image::<RgbFormat>()?;
				Ok(decoded)
			},
			Self::Callback { camera } => {
				eprintln!("Avoid calling `get_frame` on a callback camera");
				let frame = camera.poll_frame()?;
				let decoded: RgbImage = frame.decode_image::<RgbFormat>()?;
				Ok(decoded)
			},
		}
	}

	// report configurations

	// apply configurations
}
