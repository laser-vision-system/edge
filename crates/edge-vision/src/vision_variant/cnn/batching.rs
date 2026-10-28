#![expect(
	clippy::allow_attributes,
	reason = "Expect might not always be fulfilled"
)]

use burn::tensor::{
	Tensor,
	TensorData,
};
use image::{
	DynamicImage,
	GrayImage,
	imageops::FilterType,
};

use super::{
	IMG_CHANNELS_LUMA,
	config::{
		BackendDevice,
		BackendType,
	},
};
use crate::vision_variant::cnn::{
	CNN_IMG_HEIGHT,
	CNN_IMG_WIDTH,
};

#[allow(
	clippy::trivially_copy_pass_by_ref,
	reason = "Type may change with feature flag"
)]
pub(super) fn images_to_tensor<I>(
	images: I,
	device: &BackendDevice,
) -> Tensor<BackendType, 4>
where
	I: ExactSizeIterator<Item = GrayImage>,
{
	let batch_size = images.len();

	let values = images
		.map(|img| {
			DynamicImage::ImageLuma8(img)
				.resize_to_fill(
					CNN_IMG_WIDTH,
					CNN_IMG_HEIGHT,
					FilterType::Nearest,
				)
				.into_luma8()
		})
		.inspect(|img| {
			debug_assert_eq!(
				img.dimensions(),
				(CNN_IMG_WIDTH, CNN_IMG_HEIGHT),
				"The image did not have the correct dimensions"
			);
		})
		.flat_map(|img| img.to_vec())
		.collect();

	let data = TensorData::new(
		values,
		[
			batch_size,
			IMG_CHANNELS_LUMA,
			CNN_IMG_HEIGHT as usize,
			CNN_IMG_WIDTH as usize,
		],
	);
	Tensor::from_data(data, device)
}

#[cfg(test)]
mod tests {
	use image::GrayImage;

	use super::{
		super::config,
		*,
	};
	const IMAGE_PIXELS: usize = (CNN_IMG_HEIGHT * CNN_IMG_WIDTH) as usize;

	//#[test]
	// fn image_batch_tensor_is_nchw_and_keeps_images_separate() {
	// 	let images = [
	// 		RgbImage::from_pixel(
	// 			IMAGE_WIDTH,
	// 			IMAGE_HEIGHT,
	// 			image::Rgb([10, 20, 30]),
	// 		),
	// 		RgbImage::from_pixel(
	// 			IMAGE_WIDTH,
	// 			IMAGE_HEIGHT,
	// 			image::Rgb([40, 50, 60]),
	// 		),
	// 	];
	// 	let tensor =
	// 		images_to_tensor(images.into_iter(), &config::get_device());
	// 	let dimensions = tensor.dims();
	// 	let values = tensor
	// 		.into_data()
	// 		.to_vec::<f32>()
	// 		.expect("tensor values should be readable");

	// 	assert_eq!(dimensions, [2, 3, 480, 640]);
	// 	assert_eq!(values.first(), Some(&10.0));
	// 	assert_eq!(values.get(IMAGE_PIXELS), Some(&20.0));
	// 	assert_eq!(values.get(2 * IMAGE_PIXELS), Some(&30.0));
	// 	assert_eq!(values.get(3 * IMAGE_PIXELS), Some(&40.0));
	// 	assert_eq!(values.get(4 * IMAGE_PIXELS), Some(&50.0));
	// 	assert_eq!(values.get(5 * IMAGE_PIXELS), Some(&60.0));
	//}

	#[test]
	fn image_batch_tensor_is_nchw_and_keeps_images_separate() {
		let images = [
			GrayImage::from_pixel(
				CNN_IMG_WIDTH,
				CNN_IMG_HEIGHT,
				image::Luma([10]),
			),
			GrayImage::from_pixel(
				CNN_IMG_WIDTH,
				CNN_IMG_HEIGHT,
				image::Luma([40]),
			),
		];
		let tensor =
			images_to_tensor(images.into_iter(), &config::get_device());
		let dimensions = tensor.dims();
		let values = tensor
			.into_data()
			.to_vec::<f32>()
			.expect("tensor values should be readable");

		assert_eq!(dimensions, [2, 1, 60, 60]);
		assert_eq!(values.first(), Some(&10.0));
		assert_eq!(values.get(IMAGE_PIXELS), Some(&40.0));
	}
}
