use std::{env, path::PathBuf};

// Verify that only one backend feature flag is enabled at once.
#[cfg(any(
	all(
		feature = "cnn-backend-cuda",
		any(
			feature = "cnn-backend-flex",
			feature = "cnn-backend-tch",
			feature = "cnn-backend-rocm",
			feature = "cnn-backend-wgpu",
		)
	),
	all(
		feature = "cnn-backend-flex",
		any(
			feature = "cnn-backend-tch",
			feature = "cnn-backend-rocm",
			feature = "cnn-backend-wgpu",
		)
	),
	all(
		feature = "cnn-backend-tch",
		any(feature = "cnn-backend-rocm", feature = "cnn-backend-wgpu",)
	),
	all(feature = "cnn-backend-rocm", any(feature = "cnn-backend-wgpu",)),
))]
compile_error!(
	"select exactly one CNN backend feature; use --no-default-features when selecting a non-default backend"
);

#[cfg(not(any(
	feature = "cnn-backend-cuda",
	feature = "cnn-backend-flex",
	feature = "cnn-backend-tch",
	feature = "cnn-backend-rocm",
	feature = "cnn-backend-wgpu",
)))]
compile_error!("select exactly one CNN backend feature");

#[cfg(feature = "cnn-backend-cuda")]
use burn::backend::Cuda;
#[cfg(feature = "cnn-backend-flex")]
use burn::backend::Flex;
#[cfg(feature = "cnn-backend-tch")]
use burn::backend::LibTorch;
#[cfg(feature = "cnn-backend-rocm")]
use burn::backend::Rocm;
#[cfg(feature = "cnn-backend-wgpu")]
use burn::backend::Wgpu;

#[cfg(all(
	feature = "cnn-backend-cuda",
	not(any(
		feature = "cnn-backend-flex",
		feature = "cnn-backend-tch",
		feature = "cnn-backend-rocm",
		feature = "cnn-backend-wgpu",
	)),
))]
pub(super) type Backend = Cuda<f32>;
#[cfg(all(
	feature = "cnn-backend-flex",
	not(any(
		feature = "cnn-backend-tch",
		feature = "cnn-backend-rocm",
		feature = "cnn-backend-wgpu",
	))
))]
pub(super) type Backend = Flex<f32>;
#[cfg(all(
	feature = "cnn-backend-tch",
	not(any(feature = "cnn-backend-rocm", feature = "cnn-backend-wgpu",)),
))]
pub(super) type Backend = LibTorch<f32>;
#[cfg(all(feature = "cnn-backend-rocm", not(feature = "cnn-backend-wgpu")))]
pub(super) type Backend = Rocm<f32>;
#[cfg(all(feature = "cnn-backend-wgpu"))]
pub(super) type Backend = Wgpu<f32>;

pub(super) type Device = burn::prelude::Device<Backend>;

pub(super) fn get_device() -> Device {
	Device::default()
}

// TODO: Move to EnvVars.
pub(super) fn model_path() -> PathBuf {
	env::var_os("EDGE_CNN_MODEL_PATH")
		.map_or_else(|| PathBuf::from("models/laser-cnn.bpk"), PathBuf::from)
}
