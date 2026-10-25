//! Convolutional neural network.

#![expect(
	clippy::allow_attributes,
	reason = "Expect might not always be fulfilled"
)]

mod batching;
mod config;
mod model;

use std::{
	fmt,
	path::PathBuf,
};

use burn::tensor::{
	DataError,
	Tensor,
};
use burn_store::BurnpackError;
use edge_core::{
	logger::logger,
	write_optional,
};

use crate::{
	batch_item::BatchItem,
	error::{
		EdgeVisionError,
		EdgeVisionResult,
	},
	session::SessionToken,
	vision_interface::VisionInterface,
	vision_state::VisionState,
};

/// The errors that can arise with the [`Cnn`].
#[derive(Debug, thiserror::Error)]
pub enum CnnError {
	/// When a model fails to load.
	#[error("failed to load CNN model from {path}: {source}")]
	ModelLoad {
		/// The path to the model file. Not relevant if using an embedded
		/// model.
		path: PathBuf,
		/// The source error from [`BurnpackStore`](burn_store::BurnpackStore).
		#[source]
		source: BurnpackError,
	},
	/// If the prediction results are malformed.
	#[error(transparent)]
	Predict(#[from] DataError),
}

/// Closely associated data used by external holders.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ExternalData {
	/// The intensity value used for deviation measurement.
	pub setpoint: Option<f32>,
	/// The currently predicted intensity.
	pub intensity: Option<f32>,
	/// The calculated deviation value, if the setpoint exists.
	pub deviation: Option<f32>,
}

impl fmt::Display for ExternalData {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		f.write_str("ExternalData for Cnn { setpoint: ")?;
		write_optional(f, &self.setpoint)?;
		f.write_str(", intensity: ")?;
		write_optional(f, &self.intensity)?;
		f.write_str(", deviation: ")?;
		write_optional(f, &self.deviation)?;
		f.write_str(" }")
	}
}

/// A [`VisionVariant`] that uses a convolutional neural network to predict
/// intensity.
///
/// [`VisionVariant`]: super::VisionVariant
pub struct Cnn {
	session: SessionToken,
	device: config::BackendDevice,
	model: model::LaserModel<config::Backend>,
}

impl Cnn {
	/// Create and load the [`Cnn`] model.
	///
	/// # Errors
	///
	/// If the model could not be loaded correctly.
	pub fn new() -> EdgeVisionResult<Self> {
		let device = config::get_device();
		let path = config::model_path();
		let model = model::LaserModel::load(path, &device)?;
		Ok(Self {
			session: SessionToken::new(),
			device,
			model,
		})
	}

	fn predict(
		&self,
		images: Tensor<config::Backend, 4>,
	) -> EdgeVisionResult<Vec<f32>> {
		self.model
			.forward(images)
			.flatten::<1>(0, 1)
			.into_data()
			.to_vec::<f32>()
			.map_err(|why| EdgeVisionError::from(CnnError::from(why)))
	}

	#[allow(
		clippy::trivially_copy_pass_by_ref,
		reason = "Type may change with feature flag"
	)]
	fn prepare_batch(
		&self,
		batch: &mut [BatchItem],
		device: &config::BackendDevice,
	) -> Tensor<config::Backend, 4> {
		for item in batch.iter_mut() {
			self.validate_batch(item);
		}
		batching::images_to_tensor(batch.iter().map(|item| &item.image), device)
	}
}

impl VisionInterface for Cnn {
	fn process(
		&mut self,
		batch: &mut [BatchItem],
	) {
		let tensors = self.prepare_batch(batch, &self.device);
		match self.predict(tensors) {
			Ok(predictions) => apply_predictions(batch, predictions),
			Err(why) => {
				logger().log_error(&why);
				clear_predictions(batch);
			},
		}
	}

	fn new_state(&self) -> VisionState {
		VisionState::Cnn(self.session.downgrade(), ExternalData::default())
	}
}

impl fmt::Debug for Cnn {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		f.debug_struct("Cnn").finish_non_exhaustive()
	}
}

impl fmt::Display for Cnn {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		f.write_str("Cnn")
	}
}

fn apply_predictions(
	batch: &mut [BatchItem],
	predictions: Vec<f32>,
) {
	debug_assert_eq!(
		batch.len(),
		predictions.len(),
		"The number of predictions does not match the number of items"
	);

	for (item, intensity) in batch.iter_mut().zip(predictions) {
		if let VisionState::Cnn(_, data) = item.state {
			if item.calibrate {
				data.setpoint = Some(intensity);
			}
			data.intensity = Some(intensity);
			data.deviation = data
				.setpoint
				.filter(|setpoint| *setpoint > 0.0)
				.map(|setpoint| intensity / setpoint);
		}
	}
}

fn clear_predictions(batch: &mut [BatchItem]) {
	for item in batch.iter_mut() {
		if let VisionState::Cnn(_, data) = item.state {
			data.intensity = None;
			data.deviation = None;
		}
	}
}

#[cfg(test)]
mod tests {
	use std::{
		hint,
		iter,
		slice,
		time::Instant,
	};

	use edge_core::{
		image::Image,
		settings::{
			IMAGE_HEIGHT,
			IMAGE_WIDTH,
		},
	};
	use image::{
		Rgb,
		RgbImage,
	};

	use super::*;

	fn test_cnn() -> Cnn {
		let device = config::get_device();
		let model = model::LaserModel::<config::Backend>::test_model(&device);
		Cnn {
			session: SessionToken::new(),
			device,
			model,
		}
	}

	fn cnn_state(data: ExternalData) -> VisionState {
		VisionState::Cnn(SessionToken::new().downgrade(), data)
	}

	#[test]
	fn external_data_display_formats_values() {
		let data = ExternalData {
			setpoint: Some(3.5),
			intensity: Some(1.25),
			deviation: Some(0.357),
		};

		assert_eq!(
			format!("{data}"),
			"ExternalData for Cnn { setpoint: 3.5, intensity: 1.25, deviation: 0.357 }"
		);
	}

	#[test]
	fn apply_predictions_sets_intensity_and_setpoint_when_calibrating() {
		let mut state = cnn_state(ExternalData::default());
		let mut batch = [BatchItem::new(
			7,
			&mut state,
			Image::from(RgbImage::new(8, 8)),
			true,
			false,
		)];

		apply_predictions(&mut batch, vec![12.5]);

		match &batch[0].state {
			VisionState::Cnn(_, data) => {
				assert_eq!(data.setpoint, Some(12.5));
				assert_eq!(data.intensity, Some(12.5));
				assert_eq!(data.deviation, Some(1.0));
			},
			_ => panic!("expected a CNN state"),
		}
	}

	#[test]
	fn apply_predictions_uses_existing_setpoint_for_deviation() {
		let mut state = cnn_state(ExternalData {
			setpoint: Some(10.0),
			intensity: None,
			deviation: None,
		});
		let mut batch = [BatchItem::new(
			12,
			&mut state,
			Image::from(RgbImage::new(8, 8)),
			false,
			false,
		)];

		apply_predictions(&mut batch, vec![4.0]);

		match &batch[0].state {
			VisionState::Cnn(_, data) => {
				assert_eq!(data.setpoint, Some(10.0));
				assert_eq!(data.intensity, Some(4.0));
				assert_eq!(data.deviation, Some(0.4));
			},
			_ => panic!("expected a CNN state"),
		}
	}

	#[test]
	fn clear_predictions_removes_inference_values() {
		let mut state = cnn_state(ExternalData {
			setpoint: Some(9.0),
			intensity: Some(6.0),
			deviation: Some(0.666),
		});
		let mut batch = [BatchItem::new(
			21,
			&mut state,
			Image::from(RgbImage::new(8, 8)),
			false,
			true,
		)];

		clear_predictions(&mut batch);

		match &batch[0].state {
			VisionState::Cnn(_, data) => {
				assert_eq!(data.setpoint, Some(9.0));
				assert_eq!(data.intensity, None);
				assert_eq!(data.deviation, None);
			},
			_ => panic!("expected a CNN state"),
		}
	}

	#[test]
	#[ignore = "Must be run in isolation"]
	fn process_runs_inference_and_updates_state() {
		let mut cnn = test_cnn();
		let mut state = cnn_state(ExternalData::default());
		let image = Image::from(RgbImage::from_pixel(
			IMAGE_WIDTH,
			IMAGE_HEIGHT,
			Rgb([255, 40, 90]),
		));
		let mut batch = [BatchItem::new(99, &mut state, image, true, false)];

		cnn.process(&mut batch);

		match &batch[0].state {
			VisionState::Cnn(_, data) => {
				assert!(data.setpoint.is_some());
				assert!(data.intensity.is_some());
				assert!(data.deviation.is_some());
				assert!(data.intensity.unwrap().is_finite());
			},
			_ => panic!("expected a CNN state"),
		}
	}

	#[test]
	#[ignore = "manual backend profiling; run with --ignored --nocapture"]
	fn profile_process_vs_prepared_prediction() {
		const SAMPLES: u32 = 5;
		const WARMUP: u32 = 1;
		let mut cnn = Cnn::new().expect("CNN profiling requires a valid model");
		let image = Image::from(RgbImage::from_pixel(
			IMAGE_WIDTH,
			IMAGE_HEIGHT,
			Rgb([255, 40, 90]),
		));

		for batch_size in [1, 10] {
			let mut states = iter::repeat_with(|| cnn.new_state())
				.take(batch_size)
				.collect::<Vec<_>>();
			let mut batch = states
				.iter_mut()
				.enumerate()
				.map(|(index, state)| {
					BatchItem::new(
						index as u64,
						state,
						image.clone(),
						true,
						false,
					)
				})
				.collect::<Vec<_>>();
			let prepared = cnn.prepare_batch(&mut batch, &cnn.device);

			for _ in 0..WARMUP {
				let _predictions = cnn
					.predict(prepared.clone())
					.expect("prepared prediction should succeed");
				cnn.process(&mut batch);
			}

			let prediction_start = Instant::now();
			for _ in 0..SAMPLES {
				let predictions = cnn
					.predict(prepared.clone())
					.expect("prepared prediction should succeed");
				hint::black_box(predictions);
			}
			let prediction_elapsed = prediction_start.elapsed() / SAMPLES;

			let process_start = Instant::now();
			for _ in 0..SAMPLES {
				cnn.process(&mut batch);
			}
			let process_elapsed = process_start.elapsed() / SAMPLES;
			let individual_process_elapsed = (batch_size == 10).then(|| {
				let individual_start = Instant::now();
				for _ in 0..SAMPLES {
					for item in &mut batch {
						cnn.process(slice::from_mut(item));
					}
				}
				individual_start.elapsed() / SAMPLES
			});

			println!(
				"batch={batch_size}: predict_prepared={:?}, process={:?}, estimated_prepare_and_state_overhead={:?}, ten_individual_processes={individual_process_elapsed:?}",
				prediction_elapsed,
				process_elapsed,
				process_elapsed.saturating_sub(prediction_elapsed),
			);
		}
	}
}
