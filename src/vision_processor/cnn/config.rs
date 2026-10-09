use std::{
	env,
	path::{
		Path,
		PathBuf,
	},
};

// ===== BACKENDS =====
#[cfg(feature = "cnn-backend-flex")]
use burn::backend::Flex;
use burn::prelude::Device;
use burn_store::BurnpackStore;

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

pub(super) type BackendDevice = Device<Backend>;

pub(super) fn get_device() -> BackendDevice {
	BackendDevice::default()
}

// ===== MODEL SOURCE =====

#[cfg(not(feature = "cnn-model-embedded"))]
const DEFAULT_MODEL_PATH: &str = "models/laser-cnn.bpk";
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
	static MODEL_DATA: &[u8] = include_bytes!("../../../models/laser-cnn.bpk");
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
