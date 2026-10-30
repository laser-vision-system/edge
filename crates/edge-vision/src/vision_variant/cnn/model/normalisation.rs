//! A layer that provides input normalisation around known bounds.

use burn::prelude::*;

/// Configuration to create a [`Normalisation`] with the [init] function.
///
/// [init]: Self::init
#[derive(Config, Debug)]
pub struct NormalisationConfig {
	/// The lower bound of the input data.
	#[config(default = 0.0)]
	pub lower: f64,
	/// The upper bound of the input data.
	#[config(default = 255.0)]
	pub upper: f64,
}

impl NormalisationConfig {
	/// Initialises a new [`Normalisation`] module.
	pub fn init(&self) -> Normalisation {
		Normalisation {
			upper: self.upper - self.lower,
			lower: self.lower,
		}
	}
}

/// Ingests image data into latent space.
///
/// Should be created with [`NormalisationConfig`].
#[derive(Module, Debug)]
pub struct Normalisation {
	upper: f64,
	lower: f64,
}

impl Normalisation {
	/// Runs the forward pass for this layer module.
	///
	/// # Shapes
	///   - Images `[batch_size, channels, height, width]`
	///   - Output `[batch_size, channels, height, width]`
	pub fn forward(
		&self,
		input: Tensor<4>,
	) -> Tensor<4>
where {
		let rebased = input - self.lower;
		rebased / self.upper
	}
}
