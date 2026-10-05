use burn::tensor::{Tensor, TensorData};
use image::RgbImage;

use crate::vision_processor::{IMAGE_HEIGHT, IMAGE_PIXELS, IMAGE_WIDTH};

use super::config::{Backend, Device};

pub(super) fn image_to_tensor(
	image: &RgbImage,
	device: &Device,
) -> Tensor<Backend, 3> {
	debug_assert_eq!(
		image.dimensions(),
		(IMAGE_WIDTH, IMAGE_HEIGHT),
		"The image did not have the correct dimensions"
	);

	let mut red = Vec::with_capacity(IMAGE_PIXELS);
	let mut green = Vec::with_capacity(IMAGE_PIXELS);
	let mut blue = Vec::with_capacity(IMAGE_PIXELS);

	for pixel in image.pixels() {
		let image::Rgb([red_value, green_value, blue_value]) = *pixel;
		red.push(f32::from(red_value));
		green.push(f32::from(green_value));
		blue.push(f32::from(blue_value));
	}

	let values = red.into_iter().chain(green).chain(blue).collect::<Vec<_>>();
	let data = TensorData::new(values, [3, IMAGE_HEIGHT, IMAGE_WIDTH]);
	Tensor::from_data(data, device)
}

#[cfg(test)]
mod tests {
	use super::{super::config, *};

	#[test]
	fn image_tensor_is_channel_first_at_model_dimensions() {
		let image = RgbImage::from_pixel(1, 1, image::Rgb([10, 20, 30]));
		let tensor = image_to_tensor(&image, &config::get_device());
		let dimensions = tensor.dims();
		let values = tensor
			.into_data()
			.to_vec::<f32>()
			.expect("tensor values should be readable");

		assert_eq!(dimensions, [3, 480, 640]);
		assert_eq!(values.first(), Some(&10.0));
		assert_eq!(values.get(IMAGE_PIXELS), Some(&20.0));
		assert_eq!(values.get(2 * IMAGE_PIXELS), Some(&30.0));
	}
}
