#![expect(
	clippy::allow_attributes,
	reason = "Expect might not always be fulfilled"
)]

use burn::tensor::{Tensor, TensorData};

use super::config::{Backend, BackendDevice};
use crate::{
	settings::{IMAGE_HEIGHT, IMAGE_PIXELS, IMAGE_WIDTH},
	tcam_interface::Image,
};

#[allow(
	clippy::trivially_copy_pass_by_ref,
	reason = "Type may change with feature flag"
)]
pub(super) fn images_to_tensor<'a, I>(
	images: I,
	device: &BackendDevice,
) -> Tensor<Backend, 4>
where
	I: ExactSizeIterator<Item = &'a Image>,
{
	let batch_size = images.len();
	let image_size = IMAGE_PIXELS * 3;
	let mut values = vec![0; batch_size * image_size];

	for (image_values, image) in values.chunks_exact_mut(image_size).zip(images)
	{
		debug_assert_eq!(
			image.dimensions(),
			(IMAGE_WIDTH, IMAGE_HEIGHT),
			"The image did not have the correct dimensions"
		);

		let (red_values, other_channels) =
			image_values.split_at_mut(IMAGE_PIXELS);
		let (green_values, blue_values) =
			other_channels.split_at_mut(IMAGE_PIXELS);
		for (((red_slot, green_slot), blue_slot), pixel) in red_values
			.iter_mut()
			.zip(green_values)
			.zip(blue_values)
			.zip(image.pixels())
		{
			let image::Rgb([red, green, blue]) = *pixel;
			*red_slot = red;
			*green_slot = green;
			*blue_slot = blue;
		}
	}

	let data = TensorData::new(
		values,
		[batch_size, 3, IMAGE_HEIGHT as usize, IMAGE_WIDTH as usize],
	);
	Tensor::from_data(data, device)
}

#[cfg(test)]
mod tests {
	use image::RgbImage;

	use super::{super::config, *};

	#[test]
	fn image_batch_tensor_is_nchw_and_keeps_images_separate() {
		let images = [
			Image::from(RgbImage::from_pixel(
				IMAGE_WIDTH,
				IMAGE_HEIGHT,
				image::Rgb([10, 20, 30]),
			)),
			Image::from(RgbImage::from_pixel(
				IMAGE_WIDTH,
				IMAGE_HEIGHT,
				image::Rgb([40, 50, 60]),
			)),
		];
		let tensor = images_to_tensor(images.iter(), &config::get_device());
		let dimensions = tensor.dims();
		let values = tensor
			.into_data()
			.to_vec::<f32>()
			.expect("tensor values should be readable");

		assert_eq!(dimensions, [2, 3, 480, 640]);
		assert_eq!(values.first(), Some(&10.0));
		assert_eq!(values.get(IMAGE_PIXELS), Some(&20.0));
		assert_eq!(values.get(2 * IMAGE_PIXELS), Some(&30.0));
		assert_eq!(values.get(3 * IMAGE_PIXELS), Some(&40.0));
		assert_eq!(values.get(4 * IMAGE_PIXELS), Some(&50.0));
		assert_eq!(values.get(5 * IMAGE_PIXELS), Some(&60.0));
	}
}
