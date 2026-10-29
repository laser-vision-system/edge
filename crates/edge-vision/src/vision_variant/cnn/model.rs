//! Types and functions for creating a CNN model and running forward passes on
//! it.

use std::path::Path;

pub mod conv_block;
pub mod global_avg_pool;
pub mod normalisation;
pub mod read_head;

use burn::{
	nn::{
		Linear,
		LinearConfig,
	},
	prelude::*,
};

use self::{
	normalisation::{
		Normalisation,
		NormalisationConfig,
	},
	read_head::{
		ReadHead,
		ReadHeadConfig,
	},
};
use super::CnnError;
use crate::error::{
	EdgeVisionError,
	EdgeVisionResult,
};

/// Model hyper-parameters for the laser regression CNN.
#[derive(Config, Debug)]
pub(super) struct LaserModelConfig {
	/// Configuration for the stack that converts images to latent space.
	#[config(default = "ReadHeadConfig::new()")]
	pub read_head_config: ReadHeadConfig,
}

impl LaserModelConfig {
	/// Create a [`LaserModel`] with the configured hyper-parameters.
	pub fn init(
		&self,
		device: &Device,
	) -> LaserModel {
		let start_channels = self.read_head_config.start_channels;
		let double = start_channels * 2;
		let quad = start_channels * 4;
		LaserModel {
			normalisation: NormalisationConfig::new().init(),
			read_head: self.read_head_config.init(device),
			hidden: LinearConfig::new(quad, double).init(device),
			prediction: LinearConfig::new(double, 1).init(device),
		}
	}
}

/// The laser intensity prediction model.
#[derive(Module, Debug)]
pub struct LaserModel {
	normalisation: Normalisation,
	read_head: ReadHead,
	hidden: Linear,
	prediction: Linear,
}

impl LaserModel {
	pub fn load_weights(
		self,
		path: impl AsRef<Path>,
	) -> EdgeVisionResult<Self> {
		// let record = ModuleRecord::load(&path).map_err(|source| {
		// 	EdgeVisionError::from(CnnError::ModelLoad {
		// 		path: path.as_ref().to_path_buf(),
		// 		source,
		// 	})
		//})?;
		// self.try_load_record(record).map_err(|source| {
		// 	EdgeVisionError::from(CnnError::ModelLoad {
		// 		path: path.as_ref().to_path_buf(),
		// 		source,
		// 	})
		//})
		self.try_load_file(&path).map_err(|source| {
			EdgeVisionError::from(CnnError::ModelLoad {
				path: path.as_ref().to_path_buf(),
				source,
			})
		})
	}

	/// A single forward pass on the model.
	///
	/// # Shapes
	///   - Images `[batch_size, channels, height, width]`
	///   - Output `[batch_size, 1]`
	pub fn forward(
		&self,
		input: Tensor<4>,
	) -> Tensor<2> {
		let [_, channels, _, _] = input.dims();
		debug_assert_eq!(channels, 1, "expected a Luma tensor");

		let x = input; // [batch_size, 1, 640, 480]
		let x = self.normalisation.forward(x);
		let x = self.read_head.forward(x); // [batch_size, 32]
		let x = self.hidden.forward(x); // [batch_size, 32]
		self.prediction.forward(x) // [batch_size, 1]
	}
}

#[cfg(test)]
mod tests {
	use std::{
		env,
		fs,
		process,
		sync::atomic::{
			AtomicUsize,
			Ordering,
		},
	};

	use super::*;

	static NEXT_MODEL_FILE: AtomicUsize = AtomicUsize::new(0);

	#[test]
	fn model_loads_from_burnpack_file() {
		let path = env::temp_dir().join(format!(
			"edge-cnn-loader-{}-{}.bpk",
			process::id(),
			NEXT_MODEL_FILE.fetch_add(1, Ordering::Relaxed),
		));
		let device = Device::default();
		let model = LaserModelConfig::new().init(&device);
		model
			.clone()
			.save_file(&path)
			.expect("model should save to Burnpack");

		let loaded = model.load_weights(&path);
		assert!(loaded.is_ok(), "saved Burnpack should load");
		fs::remove_file(path).expect("test Burnpack should be removed");
	}
}
