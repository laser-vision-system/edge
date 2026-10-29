//! Shared types for ingesting [`nokhwa`] image buffers and encoding.
//!
//! [`FrameBuffer`] wraps the [`Buffer`] type that [`nokhwa`] returns.
//! When decoded, it is converted into an [`Image`] type, that wraps an
//! [`ImageBuffer`](image::ImageBuffer).

use std::{
	borrow::Cow,
	io::Cursor,
};

use image::{
	DynamicImage,
	ExtendedColorType,
	GenericImageView as _,
	GrayImage,
	ImageEncoder,
	RgbImage,
	codecs::{
		avif::AvifEncoder,
		jpeg::JpegEncoder,
		png::PngEncoder,
		webp::WebPEncoder,
	},
	imageops::FilterType,
};
use nokhwa::{
	Buffer,
	format_types::{
		Gray,
		Mjpeg,
		Nv12,
		RawBgr,
		RawRgb,
		Yuyv,
	},
	frame::{
		Frame,
		IntoLuma as _,
		IntoRgb as _,
		LumaConversion,
		RgbConversion,
	},
	utils::FrameFormat,
};

use crate::{
	error::{
		EdgeCoreError,
		EdgeCoreResult,
	},
	settings::{
		IMAGE_HEIGHT,
		IMAGE_WIDTH,
	},
};

/// The raw buffer of data returned from a camera device.
///
/// This hides away [`nokhwa`]'s [`Buffer`] type.
///
/// The contained data is encoded and must be decoded with
/// [`FrameBuffer::try_decode`].
#[derive(Clone, Debug)]
pub struct FrameBuffer(Buffer);

impl FrameBuffer {
	/// Consume this type and try to return [`Image`].
	///
	/// # Errors
	///
	/// If the buffer format is incorrectly labelled, malformed, corrupted, or
	/// the system is out of memory. The out of memory case is not checked and
	/// will be reported as corruption.
	pub fn try_decode_to_image(self) -> EdgeCoreResult<Image> {
		let buf: Buffer = self.into();
		let image: Image = match buf.source_frame_format() {
			FrameFormat::MJPEG => {
				Frame::<Mjpeg>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_rgb()
					.try_into()?
			},
			FrameFormat::YUYV => {
				Frame::<Yuyv>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_rgb()
					.try_into()?
			},
			FrameFormat::NV12 => {
				Frame::<Nv12>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_rgb()
					.try_into()?
			},
			FrameFormat::RAWRGB => {
				Frame::<RawRgb>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_rgb()
					.try_into()?
			},
			FrameFormat::RAWBGR => {
				Frame::<RawBgr>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_rgb()
					.try_into()?
			},
			FrameFormat::GRAY => {
				Frame::<Gray>::try_new(buf)
					.map_err(EdgeCoreError::SelectDecodeFormat)?
					.into_luma()
					.try_into()?
			},
		};
		let cropped = image.trim_excess();
		let resized = cropped.enusure_dimensions();
		Ok(resized)
	}
}

impl From<Buffer> for FrameBuffer {
	fn from(value: Buffer) -> Self {
		Self(value)
	}
}

impl From<FrameBuffer> for Buffer {
	fn from(value: FrameBuffer) -> Self {
		value.0
	}
}

/// Thin wrapper over an image buffer type provided by [`image`].
///
/// The two different image types that system can expect.
#[derive(Clone, Debug)]
pub enum Image {
	/// An image with three channels.
	Rgb(RgbImage),
	/// An image with one channels.
	Luma(GrayImage),
}

impl Image {
	/// Consume this instance and produce an [`Image::Rgb`].
	///
	/// No operation is performed if this instance is already [`Image::Rgb`].
	#[must_use]
	pub fn into_rgb(self) -> Self {
		if let Self::Luma(lumaimage) = self {
			Self::Rgb(DynamicImage::ImageLuma8(lumaimage).into_rgb8())
		} else {
			self
		}
	}

	/// Consume this instance and produce an [`Image::Luma`].
	///
	/// No operation is performed if this instance is already
	/// [`Image::Luma`].
	#[must_use]
	pub fn into_luma(self) -> Self {
		if let Self::Rgb(rgbimage) = self {
			Self::Luma(DynamicImage::ImageRgb8(rgbimage).into_luma8())
		} else {
			self
		}
	}

	/// Provide a [`Cow`] of this type.
	pub fn lazy_clone(&self) -> Cow<'_, Self> {
		Cow::Borrowed(self)
	}

	/// The width and height of this image.
	pub fn dimensions(&self) -> (u32, u32) {
		match self {
			Self::Rgb(rgbimage) => rgbimage.dimensions(),
			Self::Luma(lumaimage) => lumaimage.dimensions(),
		}
	}

	/// The width of this image.
	pub fn width(&self) -> u32 {
		match self {
			Self::Rgb(rgbimage) => rgbimage.width(),
			Self::Luma(lumaimage) => lumaimage.width(),
		}
	}

	/// The height of this image.
	pub fn height(&self) -> u32 {
		match self {
			Self::Rgb(rgbimage) => rgbimage.height(),
			Self::Luma(lumaimage) => lumaimage.height(),
		}
	}

	/// Encode the image buffer into `jpeg`.
	///
	/// # Errors
	///
	/// If the buffer could not be encoded.
	pub fn encode_to_jpeg(&self) -> EdgeCoreResult<Vec<u8>> {
		let mut buff = Cursor::new(Vec::new());
		self.encode_with(JpegEncoder::new_with_quality(&mut buff, 80))?;
		Ok(buff.into_inner())
	}

	/// Encode the image buffer into `png`.
	///
	/// # Errors
	///
	/// If the buffer could not be encoded.
	pub fn encode_to_png(&self) -> EdgeCoreResult<Vec<u8>> {
		let mut buff = Cursor::new(Vec::new());
		self.encode_with(PngEncoder::new(&mut buff))?;
		Ok(buff.into_inner())
	}

	/// Encode the image buffer into `avif`.
	///
	/// # Errors
	///
	/// If the buffer could not be encoded.
	pub fn encode_to_avif(&self) -> EdgeCoreResult<Vec<u8>> {
		let mut buff = Cursor::new(Vec::new());
		self.encode_with(AvifEncoder::new_with_speed_quality(
			&mut buff, 10, 80,
		))?;
		Ok(buff.into_inner())
	}

	/// Encode the image buffer into `webp`.
	///
	/// # Errors
	///
	/// If the buffer could not be encoded.
	pub fn encode_to_webp(&self) -> EdgeCoreResult<Vec<u8>> {
		let mut buff = Cursor::new(Vec::new());
		self.encode_with(WebPEncoder::new_lossless(&mut buff))?;
		Ok(buff.into_inner())
	}

	/// Remove additional data that may be present outside the frame.
	#[must_use]
	fn trim_excess(self) -> Self {
		match self {
			Self::Rgb(rgbimage) => {
				let frame = DynamicImage::ImageRgb8(rgbimage);
				let trimmed = trim_excess(&frame);
				Self::from(trimmed.into_rgb8())
			},
			Self::Luma(lumaimage) => {
				let frame = DynamicImage::ImageLuma8(lumaimage);
				let trimmed = trim_excess(&frame);
				Self::from(trimmed.into_luma8())
			},
		}
	}

	/// Make sure this image has the expected dimensions.
	#[must_use]
	fn enusure_dimensions(self) -> Self {
		match self {
			Self::Rgb(rgbimage) => {
				let frame = DynamicImage::ImageRgb8(rgbimage);
				let cropped = enusure_dimensions(frame);
				Self::from(cropped.into_rgb8())
			},
			Self::Luma(lumaimage) => {
				let frame = DynamicImage::ImageLuma8(lumaimage);
				let cropped = enusure_dimensions(frame);
				Self::from(cropped.into_luma8())
			},
		}
	}

	fn encode_with(
		&self,
		encoder: impl ImageEncoder,
	) -> EdgeCoreResult {
		encoder
			.write_image(
				self.as_ref(),
				self.width(),
				self.height(),
				self.as_colour_type(),
			)
			.map_err(EdgeCoreError::from)
	}

	fn as_colour_type(&self) -> ExtendedColorType {
		match self {
			Self::Rgb(_) => ExtendedColorType::Rgb8,
			Self::Luma(_) => ExtendedColorType::L8,
		}
	}
}

impl AsRef<[u8]> for Image {
	fn as_ref(&self) -> &[u8] {
		match self {
			Self::Rgb(rgbimage) => rgbimage,
			Self::Luma(lumaimage) => lumaimage,
		}
	}
}

impl AsMut<[u8]> for Image {
	fn as_mut(&mut self) -> &mut [u8] {
		match self {
			Self::Rgb(rgbimage) => rgbimage,
			Self::Luma(lumaimage) => lumaimage,
		}
	}
}

impl From<DynamicImage> for Image {
	fn from(value: DynamicImage) -> Self {
		// Pick the greediest option to avoid losing data.
		Self::Rgb(value.into_rgb8())
	}
}

impl From<RgbImage> for Image {
	fn from(value: RgbImage) -> Self {
		Self::Rgb(value)
	}
}

impl From<GrayImage> for Image {
	fn from(value: GrayImage) -> Self {
		Self::Luma(value)
	}
}

impl TryFrom<RgbConversion> for Image {
	type Error = EdgeCoreError;

	fn try_from(value: RgbConversion) -> EdgeCoreResult<Self> {
		let image = value
			.materialize()
			// We just assume it is always corruption for now.
			// TODO: Figure out the cause.
			// If it is because of memory, maybe try unwind and restart.
			.map_err(EdgeCoreError::FrameDecodeCorruption)?;
		Ok(Self::from(image))
	}
}

impl TryFrom<LumaConversion> for Image {
	type Error = EdgeCoreError;

	fn try_from(value: LumaConversion) -> EdgeCoreResult<Self> {
		let image = value
			.materialize()
			// We just assume it is always corruption for now.
			// TODO: Figure out the cause.
			// If it is because of memory, maybe try unwind and restart.
			.map_err(EdgeCoreError::FrameDecodeCorruption)?;
		Ok(Self::from(image))
	}
}

impl TryFrom<FrameBuffer> for Image {
	type Error = EdgeCoreError;

	fn try_from(buffer: FrameBuffer) -> EdgeCoreResult<Self> {
		buffer.try_decode_to_image()
	}
}

impl From<Image> for RgbImage {
	fn from(value: Image) -> Self {
		match value {
			Image::Rgb(rgbimage) => rgbimage,
			Image::Luma(lumaimage) => {
				DynamicImage::ImageLuma8(lumaimage).into_rgb8()
			},
		}
	}
}

impl From<Image> for GrayImage {
	fn from(value: Image) -> Self {
		match value {
			Image::Rgb(rgbimage) => {
				DynamicImage::ImageRgb8(rgbimage).into_luma8()
			},
			Image::Luma(lumaimage) => lumaimage,
		}
	}
}

impl<'a> From<&'a Image> for &'a [u8] {
	fn from(value: &'a Image) -> Self {
		match value {
			Image::Rgb(rgbimage) => rgbimage.as_ref(),
			Image::Luma(lumaimage) => lumaimage.as_ref(),
		}
	}
}

fn enusure_dimensions(image: DynamicImage) -> DynamicImage {
	if image.dimensions() == (IMAGE_WIDTH, IMAGE_HEIGHT) {
		image
	} else {
		// TODO: Check which type of resize method we want to use.
		image.resize_exact(IMAGE_WIDTH, IMAGE_HEIGHT, FilterType::Triangle)
	}
}

fn trim_excess(image: &DynamicImage) -> DynamicImage {
	image.crop_imm(0, 0, image.width(), image.height())
}
