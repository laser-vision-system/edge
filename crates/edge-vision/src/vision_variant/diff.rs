// TODO: Remove all unwraps and then this.
#![expect(clippy::unwrap_used, reason = "WIP development")]

use std::fmt;

use edge_core::logger::logger;
use image::{
	DynamicImage,
	GrayImage,
};
use image_compare::{
	Algorithm,
	Metric,
	gray_similarity_histogram,
	gray_similarity_structure,
};

use crate::{
	batch_item::BatchItem,
	session::SessionToken,
	vision_interface::VisionInterface,
	vision_state::VisionState,
};

struct DiffModel {
	intercept: f32,
	peak: f32,
	mean: f32,
	std_dev: f32,
	luma_ssim: f32,
	hellinger: f32,
	correlation: f32,
	intersection: f32,
}

impl DiffModel {
	#[expect(
		clippy::too_many_arguments,
		reason = "Necessary for this function"
	)]
	fn predict(
		&self,
		peak: f32,
		mean: f32,
		std_dev: f32,
		luma_ssim: f32,
		hellinger: f32,
		correlation: f32,
		intersection: f32,
	) -> f32 {
		self.intercept
			+ self.peak * peak
			+ self.mean * mean
			+ self.std_dev * std_dev
			+ self.luma_ssim * luma_ssim
			+ self.hellinger * hellinger
			+ self.correlation * correlation
			+ self.intersection * intersection
	}
}

impl Default for DiffModel {
	#[expect(
		clippy::excessive_precision,
		clippy::unreadable_literal,
		reason = "These are raw model weights"
	)]
	fn default() -> Self {
		DiffModel {
			intercept: 0.5819997604631103,
			peak: -0.027890896459691977,
			mean: 0.03606082141430921,
			std_dev: -0.21063533527616005,
			luma_ssim: 0.024281740506804317,
			hellinger: -0.6446002924029192,
			correlation: -0.042378884592307826,
			intersection: -0.5703440858771205,
		}
	}
}

#[derive(Default)]
pub struct Diff {
	session: SessionToken,
	model: DiffModel,
}

impl Diff {
	pub fn new() -> Self {
		Self {
			session: SessionToken::new(),
			model: DiffModel::default(),
		}
	}

	#[expect(
		clippy::cast_precision_loss,
		reason = "The pixel count should never exceed the limits of a f32"
	)]
	fn compute_brightness_stats(img: &GrayImage) -> (u8, f32, f32) {
		let pixels: Vec<u8> = img.pixels().map(|pixel| pixel.0[0]).collect();
		let n = pixels.len() as f32;

		let sum: f32 = pixels.iter().map(|&pixel| f32::from(pixel)).sum();
		let mean = sum / n;

		let variance = pixels
			.iter()
			.map(|&pixel| {
				let dev = f32::from(pixel) - mean;
				dev * dev
			})
			.sum::<f32>()
			/ n;

		let std_dev = variance.sqrt();

		let peak = pixels.iter().max().copied().unwrap_or(0);

		(peak, mean, std_dev)
	}

	#[expect(
		clippy::cast_possible_truncation,
		reason = "Value is used at f32, so full size not needed"
	)]
	fn collect_metrics(
		image: &GrayImage,
		setpoint: &GrayImage,
	) -> (f32, f32, f32, f32, f32, f32, f32) {
		let (image_peak, image_mean, image_std_dev) =
			Self::compute_brightness_stats(image);
		let (setpoint_peak, setpoint_mean, setpoint_std_dev) =
			Self::compute_brightness_stats(setpoint);
		let luma_ssim =
			gray_similarity_structure(&Algorithm::MSSIMSimple, image, setpoint)
				.unwrap()
				.score as f32;
		let hellinger =
			gray_similarity_histogram(Metric::Hellinger, image, setpoint)
				.unwrap() as f32;
		let correlation =
			gray_similarity_histogram(Metric::Correlation, image, setpoint)
				.unwrap() as f32;
		let intersection =
			gray_similarity_histogram(Metric::Intersection, image, setpoint)
				.unwrap() as f32;

		(
			f32::from(setpoint_peak) - f32::from(image_peak),
			setpoint_mean - image_mean,
			setpoint_std_dev - image_std_dev,
			luma_ssim,
			hellinger,
			correlation,
			intersection,
		)
	}
}

impl VisionInterface for Diff {
	fn process(
		&mut self,
		batch: &mut [BatchItem],
	) {
		for item in batch {
			let VisionState::Diff {
				data, ..
			} = item.state
			else {
				// TODO: This should raise an error.
				continue;
			};

			if item.calibrate {
				let grey_image =
					DynamicImage::ImageRgb8((*item.image).clone()).into_luma8();
				data.image = Some(grey_image.clone());
				let (peak, mean, std_dev) =
					Self::compute_brightness_stats(&grey_image);
				data.peak = Some(f32::from(peak));
				data.mean = Some(mean);
				data.std_dev = Some(std_dev);
				continue;
			}

			if let Some(setpoint) = &data.image {
				let grey_image =
					DynamicImage::ImageRgb8((*item.image).clone()).into_luma8();

				let (
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
				) = Self::collect_metrics(&grey_image, setpoint);

				let relative_intensity = self.model.predict(
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
				);

				data.deviation = Some(1.0 - relative_intensity);
			} else {
				logger().lock().unwrap().log_warn(
					"Could not calculate deviation as there is no setpoint!",
				);
			}
		}
	}

	fn new_state(&self) -> VisionState {
		VisionState::Diff {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}
}

impl fmt::Debug for Diff {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "Diff")
	}
}

impl fmt::Display for Diff {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "Diff")
	}
}

/// Closely associated data used by external holders.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExternalData {
	pub image: Option<GrayImage>,
	pub peak: Option<f32>,
	pub mean: Option<f32>,
	pub std_dev: Option<f32>,
	pub deviation: Option<f32>,
}

impl fmt::Display for ExternalData {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(
			f,
			"ExternalData for Diff\n - peak: {:?}\n - mean: {:?}\n - std_dev: {:?}",
			self.peak, self.mean, self.std_dev
		)
	}
}
