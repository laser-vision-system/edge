use std::{
	env,
	path::{
		Path,
		PathBuf,
	},
};

use burn::prelude::Device;
use burn_store::BurnpackStore;

// ===== BACKENDS =====

#[cfg(feature = "cnn-backend-flex")]
mod backend {
	use burn::backend::Flex;
	pub(crate) type BackendType = Flex<f32>;
}

#[cfg(feature = "cnn-backend-wgpu")]
mod backend {
	use burn::backend::Wgpu;
	pub(crate) type BackendType = Wgpu<f32>;
}

#[cfg(feature = "cnn-backend-cuda")]
mod backend {
	use burn::backend::Cuda;
	pub(crate) type BackendType = Cuda<f32>;
}

pub(super) use backend::*;

pub(super) type BackendDevice = Device<BackendType>;

pub(super) fn get_device() -> BackendDevice {
	BackendDevice::default()
}

// ===== MODEL SOURCE =====

#[cfg(not(feature = "cnn-model-embedded"))]
const DEFAULT_MODEL_PATH: &str = "../../models/laser-cnn.bpk";
#[cfg(feature = "cnn-model-embedded")]
const DEFAULT_MODEL_PATH: &str = "USING EMBEDDED MODEL";

// TODO: Move to EnvVars.
pub(super) fn model_path() -> PathBuf {
	env::var_os("EDGE_CNN_MODEL_PATH")
		.map_or_else(|| PathBuf::from(DEFAULT_MODEL_PATH), PathBuf::from)
}

pub fn get_burnpack<P>(path: P) -> BurnpackStore
where
	P: AsRef<Path>,
{
	#[cfg(feature = "cnn-model-embedded")]
	return store_from_embedded(path);

	#[cfg(feature = "cnn-model-file")]
	return store_from_file(path);

	#[cfg(feature = "cnn-model-zero-copy")]
	return store_from_zero_copy(path);
}

#[cfg(feature = "cnn-model-embedded")]
fn store_from_embedded<P>(_: P) -> BurnpackStore
where
	P: AsRef<Path>,
{
	static MODEL_DATA: &[u8] =
		include_bytes!("../../../../../models/laser-cnn.bpk");
	BurnpackStore::from_static(MODEL_DATA)
}

#[cfg(feature = "cnn-model-file")]
fn store_from_file<P>(path: P) -> BurnpackStore
where
	P: AsRef<Path>,
{
	BurnpackStore::from_file(&path)
}

#[cfg(feature = "cnn-model-zero-copy")]
fn store_from_zero_copy<P>(path: P) -> BurnpackStore
where
	P: AsRef<Path>,
{
	BurnpackStore::from_file(&path).zero_copy(true)
}
