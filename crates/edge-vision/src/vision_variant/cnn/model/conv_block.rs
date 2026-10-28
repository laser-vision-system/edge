//! Custom layer module for convolution layers.

use burn::{
	nn::{
		BatchNorm,
		BatchNormConfig,
		PaddingConfig2d,
		conv::{
			Conv2d,
			Conv2dConfig,
		},
	},
	prelude::*,
	tensor::activation::{
		hard_swish,
		leaky_relu,
		relu,
	},
};
use serde::{
	Deserialize,
	Serialize,
};

/// Selection for the activation type of a [`ConvBlock`].
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum ConvBlockActivation {
	/// Use the [`relu`] activation function.
	#[default]
	Relu,
	/// Use the [`hard_swish`] activation function.
	HardSwish,
	/// Use the [`leaky_relu`] activation function.
	LeakyRelu(f64),
}

/// Configuration to create a [`ConvBlock`] with the [init] function.
///
/// [init]: Self::init
#[derive(Config, Debug)]
pub struct ConvBlockConfig {
	/// The number of input channels.
	pub input_channels: usize,
	/// The number of output channels.
	pub output_channels: usize,
	/// The activation function to use for the output.
	#[config(default = "ConvBlockActivation::Relu")]
	pub activation: ConvBlockActivation,
	/// The size of the kernel.
	#[config(default = 3)]
	pub kernel_size: usize,
	/// The stride of the convolution (squared).
	#[config(default = 1)]
	pub stride: usize,
	/// The number of padding pixels added to the edge.
	#[config(default = 1)]
	pub padding: usize,
	/// Controls the exponential moving average of the mean and variance
	/// estimates used to update the metrics in batch normalisation.
	#[config(default = 0.1)]
	pub momentum: f64,
	// ---
	// Copied from Conv2dConfig:
	// ---
	/// Spacing between kernel elements.
	#[config(default = "[1, 1]")]
	pub dilation: [usize; 2],
	/// Controls the connections between input and output channels.
	#[config(default = "1")]
	pub groups: usize,
	/// If bias should be added to the output.
	#[config(default = true)]
	pub bias: bool,
	// ---
	// Copied from BatchNormConfig:
	// ---
	/// A value required for numerical stability.
	#[config(default = 1e-5)]
	pub epsilon: f64,
}

impl ConvBlockConfig {
	/// Initialises a new [`ConvBlock`] module.
	pub fn init(
		&self,
		device: &Device,
	) -> ConvBlock {
		ConvBlock {
			conv: Conv2dConfig::new(
				[self.input_channels, self.output_channels],
				[self.kernel_size; 2],
			)
			.with_stride([self.stride; 2])
			.with_dilation(self.dilation)
			.with_groups(self.groups)
			.with_padding(PaddingConfig2d::Explicit(
				self.padding,
				self.padding,
				self.padding,
				self.padding,
			))
			.with_bias(self.bias)
			.init(device),
			batch_norm: BatchNormConfig::new(self.output_channels)
				.with_epsilon(self.epsilon)
				.with_momentum(self.momentum)
				.init(device),
			activation: self.activation,
		}
	}
}

/// A chain of a [`Conv2d`], [`BatchNorm`], and activation function.
///
/// Should be created with [`ConvBlockConfig`].
#[derive(Module, Debug)]
pub struct ConvBlock {
	conv: Conv2d,
	batch_norm: BatchNorm,
	#[module(skip)]
	activation: ConvBlockActivation,
}

impl ConvBlock {
	/// Runs the forward pass for this layer module.
	///
	/// # Shapes
	///   - Images `[batch_size, channels, height, width]`
	///   - Output `[batch_size, channels, height, width]`
	pub fn forward(
		&self,
		input: Tensor<4>,
	) -> Tensor<4> {
		let x = input;
		let x = self.conv.forward(x);
		let x = self.batch_norm.forward(x);
		match self.activation {
			ConvBlockActivation::Relu => relu(x),
			ConvBlockActivation::HardSwish => hard_swish(x),
			ConvBlockActivation::LeakyRelu(neg_slope) => {
				leaky_relu(x, neg_slope)
			},
		}
	}
}
