use std::{
	env,
	path::PathBuf,
};

// ===== MODEL SOURCE =====

const DEFAULT_MODEL_PATH: &str = "./models/laser-cnn.bpk";

// TODO: Move to EnvVars.
pub(super) fn model_path() -> PathBuf {
	env::var_os("EDGE_CNN_MODEL_PATH")
		.map_or_else(|| PathBuf::from(DEFAULT_MODEL_PATH), PathBuf::from)
}
