#![expect(clippy::module_name_repetitions, reason = "Not a problem")]

use std::path::Path;

use burn::{
	nn::{
		BatchNorm, BatchNormConfig, Linear, LinearConfig, PaddingConfig2d,
		conv::{Conv2d, Conv2dConfig},
		pool::{
			AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig, MaxPool2d,
			MaxPool2dConfig,
		},
	},
	prelude::*,
	tensor::activation::relu,
};
use burn_store::ModuleSnapshot as _;

use super::{CnnError, config};
use crate::{
	error::{EdgeError, Result},
	vision_processor::VisionError,
};

#[derive(Module, Debug)]
pub struct LaserModel<B: Backend> {
	read_head: Conv2d<B>,
	read_head_norm: BatchNorm<B>,
	read_head_pool: MaxPool2d,
	detail_extraction: Conv2d<B>,
	detail_extraction_norm: BatchNorm<B>,
	detail_extraction_pool: MaxPool2d,
	// Burn does not support GlobalAveragePooling2D
	avg_pool: AdaptiveAvgPool2d,
	prediction: Linear<B>,
}

impl<B: Backend> LaserModel<B> {
	fn init(device: &B::Device) -> Self {
		Self {
			read_head: Conv2dConfig::new([3, 16], [3, 3])
				.with_padding(PaddingConfig2d::Same)
				.init(device),
			read_head_norm: BatchNormConfig::new(16)
				.with_momentum(0.1)
				.init(device),
			read_head_pool: MaxPool2dConfig::new([4, 4]).init(),
			detail_extraction: Conv2dConfig::new([16, 8], [3, 3])
				.with_padding(PaddingConfig2d::Same)
				.init(device),
			detail_extraction_norm: BatchNormConfig::new(8)
				.with_momentum(0.1)
				.init(device),
			detail_extraction_pool: MaxPool2dConfig::new([4, 4]).init(),
			avg_pool: AdaptiveAvgPool2dConfig::new([1, 1]).init(),
			prediction: LinearConfig::new(8, 1).init(device),
		}
	}

	#[cfg(test)]
	pub(crate) fn test_model(device: &B::Device) -> Self {
		Self::init(device)
	}

	pub fn load<P>(path: P, device: &B::Device) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		let mut model = Self::init(device);

		let path_buf = path.as_ref().to_path_buf();
		let mut store = config::get_burnpack(path);
		let _apply_result = model.load_from(&mut store).map_err(|source| {
			EdgeError::from(VisionError::from(CnnError::ModelLoad {
				path: path_buf,
				source,
			}))
		})?;
		Ok(model)
	}

	/// A single forward pass on the model.
	///
	/// # Shapes
	///   - Images \[`batch_size`, channels, height, width\]
	///   - Output \[`batch_size`, 1\]
	pub fn forward(&self, images: Tensor<B, 4>) -> Tensor<B, 2> {
		let [batch_size, channels, _, _] = images.dims();
		debug_assert_eq!(channels, 3, "expected an RGB tensor");

		let x = images;
		let x = self.read_head.forward(x); // [batch_size, 16, 640, 480]
		let x = self.read_head_norm.forward(x);
		let x = relu(x);
		let x = self.read_head_pool.forward(x); // [batch_size, 16, 160, 120]
		let x = self.detail_extraction.forward(x); // [batch_size, 8, 160, 120]
		let x = self.detail_extraction_norm.forward(x);
		let x = relu(x);
		let x = self.detail_extraction_pool.forward(x); // [batch_size, 8, 40, 30]
		// These two layers function as the `GlobalAveragePooling2D` layer and
		// drop the shape to two dimensions.
		let x = self.avg_pool.forward(x); // [batch_size, 8, 1, 1]
		let x = x.reshape([batch_size, 8]); // [batch_size, 8]
		self.prediction.forward(x) // [batch_size, 1]
	}
}

#[cfg(test)]
mod tests {
	use std::{
		fs,
		sync::atomic::{AtomicUsize, Ordering},
	};

	use burn_store::BurnpackStore;

	use super::{super::config, *};

	static NEXT_MODEL_FILE: AtomicUsize = AtomicUsize::new(0);

	#[test]
	fn model_loads_from_burnpack_file() {
		let path = std::env::temp_dir().join(format!(
			"edge-cnn-loader-{}-{}.bpk",
			std::process::id(),
			NEXT_MODEL_FILE.fetch_add(1, Ordering::Relaxed),
		));
		let device = config::get_device();
		let model = LaserModel::<config::Backend>::init(&device);
		let mut store = BurnpackStore::from_file(&path).overwrite(true);
		model
			.save_into(&mut store)
			.expect("model should save to Burnpack");

		let loaded = LaserModel::<config::Backend>::load(&path, &device);
		assert!(loaded.is_ok(), "saved Burnpack should load");
		fs::remove_file(path).expect("test Burnpack should be removed");
	}
}
