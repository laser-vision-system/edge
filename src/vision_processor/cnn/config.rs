use std::{env, path::PathBuf};

// Verify that only one backend feature flag is enabled at once.
#[cfg(not(any(
	feature = "cnn-backend-flex",
	feature = "cnn-backend-cuda",
	feature = "cnn-backend-wgpu",
)))]
compile_error!("select exactly one CNN backend feature");

#[cfg(any(
	all(
		feature = "cnn-backend-flex",
		any(feature = "cnn-backend-wgpu", feature = "cnn-backend-cuda",)
	),
	all(feature = "cnn-backend-wgpu", any(feature = "cnn-backend-cuda")),
))]
compile_error!(
	"select exactly one CNN backend feature; use --no-default-features when selecting a non-default backend"
);

#[cfg(feature = "cnn-backend-flex")]
use burn::backend::Flex;

#[cfg(feature = "cnn-backend-flex")]
pub(super) type Backend = Flex<f32>;

#[cfg(feature = "cnn-backend-wgpu")]
use burn::backend::Wgpu;

#[cfg(all(
	feature = "cnn-backend-wgpu",
	not(any(feature = "cnn-backend-flex", feature = "cnn-backend-cuda"))
))]
pub(super) type Backend = Wgpu<f32>;

#[cfg(feature = "cnn-backend-cuda")]
use burn::backend::Cuda;

#[cfg(all(
	feature = "cnn-backend-cuda",
	not(any(feature = "cnn-backend-flex", feature = "cnn-backend-wgpu"))
))]
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
