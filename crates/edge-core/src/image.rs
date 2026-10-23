use std::{
	borrow::Cow,
	io::Cursor,
	ops::{
		Deref,
		DerefMut,
	},
};

use image::{
	DynamicImage,
	GenericImageView as _,
	RgbImage,
	codecs::jpeg,
	imageops::FilterType,
};
use nokhwa::{
	Buffer,
	NokhwaError,
	format_types::{
		Mjpeg,
		Nv12,
		RawBgr,
		RawRgb,
		Yuyv,
	},
	frame::{
		Frame,
		IntoRgb as _,
		RgbConversion,
	},
	utils::FrameFormat,
};

use crate::{
	error::{
		EdgeCoreError,
		EdgeCoreResult,
		FrameDecodeCorruption,
		SelectDecodeFormat,
	},
	settings::{
		IMAGE_HEIGHT,
		IMAGE_WIDTH,
	},
};

#[derive(Clone, Debug)]
pub struct FrameBuffer {
	inner: Buffer,
}

impl FrameBuffer {
	/// Decode this type into an image.
	///
	/// # Errors
	///
	/// If the frame is corrupted or the system is out of memory.
	/// The out of memory case is not checked, so only the corruption errors is
	/// reported.
	pub fn try_decode(self) -> EdgeCoreResult<Image> {
		let decoded_res = select_decode_format(self.inner)?;

		let img_buffer = decoded_res
			.materialize()
			// We just assume it is always corruption for now.
			// TODO: Figure out the why.
			// If it is because of memory, maybe try unwind and restart.
			.map_err(|why| {
				EdgeCoreError::from(FrameDecodeCorruption::from(why))
			})?;

		let frame = DynamicImage::ImageRgb8(img_buffer);
		let cropped = frame.crop_imm(0, 0, frame.width(), frame.height());
		let resized = enusure_dimensions(cropped).into_rgb8();
		Ok(Image::from(resized))
	}
}

impl From<Buffer> for FrameBuffer {
	fn from(value: Buffer) -> Self {
		Self {
			inner: value,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Image {
	inner: RgbImage,
}

impl Image {
	pub fn lazy_clone(&self) -> Cow<'_, Self> {
		Cow::Borrowed(self)
	}

	#[expect(clippy::unwrap_used, reason = "WIP development")]
	/// Encode the image buffer into jpeg.
	///
	/// # Panics
	///
	/// If the buffer could not be encoded.
	pub fn encode(&self) -> Vec<u8> {
		let mut buff = Cursor::new(Vec::new());
		let mut enc = jpeg::JpegEncoder::new(&mut buff);
		enc.encode_image(&self.inner).unwrap();
		buff.into_inner()
	}
}

impl Deref for Image {
	type Target = RgbImage;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Image {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl From<RgbImage> for Image {
	fn from(value: RgbImage) -> Self {
		Self {
			inner: value,
		}
	}
}

/// Helper function to choose the decode method.
fn select_decode_format(buf: Buffer) -> EdgeCoreResult<RgbConversion> {
	match buf.source_frame_format() {
		FrameFormat::MJPEG => {
			Ok(Frame::<Mjpeg>::try_new(buf)
				.map_err(|why| {
					EdgeCoreError::from(SelectDecodeFormat::from(why))
				})?
				.into_rgb())
		},
		FrameFormat::YUYV => {
			Ok(Frame::<Yuyv>::try_new(buf)
				.map_err(|why| {
					EdgeCoreError::from(SelectDecodeFormat::from(why))
				})?
				.into_rgb())
		},
		FrameFormat::NV12 => {
			Ok(Frame::<Nv12>::try_new(buf)
				.map_err(|why| {
					EdgeCoreError::from(SelectDecodeFormat::from(why))
				})?
				.into_rgb())
		},
		FrameFormat::RAWRGB => {
			Ok(Frame::<RawRgb>::try_new(buf)
				.map_err(|why| {
					EdgeCoreError::from(SelectDecodeFormat::from(why))
				})?
				.into_rgb())
		},
		FrameFormat::RAWBGR => {
			Ok(Frame::<RawBgr>::try_new(buf)
				.map_err(|why| {
					EdgeCoreError::from(SelectDecodeFormat::from(why))
				})?
				.into_rgb())
		},
		// TODO: Support this type of input
		FrameFormat::GRAY => {
			let nok_err = NokhwaError::general("GRAY/Luma cameras not support");
			let err = EdgeCoreError::from(SelectDecodeFormat::from(nok_err));
			Err(err)
		},
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

//#[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn decode_image() {
// 		let frame = FrameBuffer;
// 		_ = frame.decode();
// 	}
//}
