use std::{env, path::PathBuf};

#[cfg(feature = "cnn-backend-flex")]
use burn::backend::Flex;

#[cfg(feature = "cnn-backend-flex")]
pub(super) type Backend = Flex<f32>;

#[cfg(feature = "cnn-backend-wgpu")]
use burn::backend::Wgpu;

#[cfg(feature = "cnn-backend-wgpu")]
pub(super) type Backend = Wgpu<f32>;

#[cfg(feature = "cnn-backend-cuda")]
use burn::backend::Cuda;

#[cfg(feature = "cnn-backend-cuda")]
pub(super) type Backend = Cuda<f32>;

pub(super) type Device = burn::prelude::Device<Backend>;

pub(super) fn get_device() -> Device {
	Device::default()
}

// TODO: Move to EnvVars.
pub(super) fn model_path() -> PathBuf {
	env::var_os("EDGE_CNN_MODEL_PATH")
		.map_or_else(|| PathBuf::from("models/laser-cnn.bpk"), PathBuf::from)
}
