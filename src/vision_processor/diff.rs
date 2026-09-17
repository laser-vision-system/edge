use std::fmt;

use image::{DynamicImage, GrayImage};

use image_compare::{Algorithm, Metric, gray_similarity_histogram, gray_similarity_structure};

use super::{BatchItem, SessionToken, VisionInterface, VisionState};

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

	pub fn new_state(&self) -> VisionState {
		VisionState::Diff {
			handle: self.session.downgrade(),
			data: ExternalData::default(),
		}
	}

	fn compute_brightness_stats(&self, img: &GrayImage) -> (u8, f32, f32) {
		let pixels: Vec<u8> = img.pixels().map(|p| p.0[0]).collect();
		let n = pixels.len() as f32;

		let sum: f32 = pixels.iter().map(|&p| p as f32).sum();
		let mean = sum / n;

		let variance = pixels
			.iter()
			.map(|&p| {
				let d = p as f32 - mean;
				d * d
			})
			.sum::<f32>()
			/ n;

		let std_dev = variance.sqrt();

		let peak = pixels.iter().max().copied().unwrap_or(0);

		(peak, mean, std_dev)
	}

	fn collect_metrics(
		&self,
		image: &GrayImage,
		setpoint: &GrayImage,
	) -> (f32, f32, f32, f32, f32, f32, f32) {
		let (image_peak, image_mean, image_std_dev) = self.compute_brightness_stats(image);
		let (setpoint_peak, setpoint_mean, setpoint_std_dev) =
			self.compute_brightness_stats(setpoint);
		let luma_ssim = gray_similarity_structure(&Algorithm::MSSIMSimple, image, setpoint)
			.unwrap()
			.score as f32;
		let hellinger =
			gray_similarity_histogram(Metric::Hellinger, image, setpoint).unwrap() as f32;
		let correlation =
			gray_similarity_histogram(Metric::Correlation, image, setpoint).unwrap() as f32;
		let intersection =
			gray_similarity_histogram(Metric::Intersection, image, setpoint).unwrap() as f32;

		return (
			setpoint_peak as f32 - image_peak as f32,
			setpoint_mean - image_mean,
			setpoint_std_dev - image_std_dev,
			luma_ssim,
			hellinger,
			correlation,
			intersection,
		);
	}
}

impl VisionInterface for Diff {
	fn process(&mut self, batch: &mut [BatchItem]) {
		for item in batch {
			let VisionState::Diff { data, .. } = &mut item.state else {
				// TODO: This should raise an error.
				continue;
			};

			if item.calibrate {
				let grey_image = Some(DynamicImage::ImageRgb8(item.image.clone()).into_luma8());
				data.image = grey_image.clone();
				let (peak, mean, std_dev) = self.compute_brightness_stats(&grey_image.unwrap());
				data.peak = Some(peak as f32);
				data.mean = Some(mean);
				data.std_dev = Some(std_dev);
				continue;
			}

			match &data.image {
				Some(setpoint) => {
					let grey_image = DynamicImage::ImageRgb8(item.image.clone()).into_luma8();

					let (
						peak_diff,
						mean_diff,
						std_dev_diff,
						luma_ssim,
						hellinger,
						correlation,
						intersection,
					) = self.collect_metrics(&grey_image, setpoint);

					let relative_intensity = self.model.predict(
						peak_diff,
						mean_diff,
						std_dev_diff,
						luma_ssim,
						hellinger,
						correlation,
						intersection,
					);

					data.deviation = Some(1.0 - relative_intensity as f32);
				},
				None => {
					logger::logger().log_warn("Could not calculate deviation as there is no setpoint!");
					continue;
				},
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

impl Default for Diff {
	fn default() -> Self {
		Self {
			session: SessionToken::new(),
			model: DiffModel::default(),
		}
	}
}

impl fmt::Debug for Diff {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Diff")
	}
}

impl fmt::Display for Diff {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"ExternalData for Diff\n - peak: {:?}\n - mean: {:?}\n - std_dev: {:?}",
			self.peak, self.mean, self.std_dev
		)
	}
}
