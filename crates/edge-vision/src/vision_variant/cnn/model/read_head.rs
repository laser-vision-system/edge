//! The stack of convolutions to convert an input image into a latent vector.
//!
//! Similar to a tokeniser for an LLM.

use burn::{
	nn::pool::{
		MaxPool2d,
		MaxPool2dConfig,
	},
	prelude::*,
};

use super::{
	conv_block::{
		ConvBlock,
		ConvBlockConfig,
	},
	global_avg_pool::GlobalAvgPooling2d,
};

/// Configuration to create a [`ReadHead`] with the [init] function.
///
/// Output channels will be `start_channels` * 4.
///
/// [init]: Self::init
#[derive(Config, Debug)]
pub struct ReadHeadConfig {
	/// How many channels the input image has.
	#[config(default = 1)]
	pub input_channels: usize,
	/// How many channels the first step of convolution produces.
	///
	/// The output channels will be four times this.
	#[config(default = 8)]
	pub start_channels: usize,
	/// The batch normalisation momentum for stage 1.
	#[config(default = 0.1)]
	pub stage_1_momentum: f64,
	/// The batch normalisation momentum for stage 2a.
	#[config(default = 0.1)]
	pub stage_2a_momentum: f64,
	/// The batch normalisation momentum for stage 2b.
	#[config(default = 0.1)]
	pub stage_2b_momentum: f64,
	/// The batch normalisation momentum for stage 3.
	#[config(default = 0.1)]
	pub stage_3_momentum: f64,
}

impl ReadHeadConfig {
	/// Initialises a new [`ReadHead`] module.
	pub fn init<B>(
		&self,
		device: &B::Device,
	) -> ReadHead<B>
	where
		B: Backend,
	{
		let stage1_out = self.start_channels;
		let stage2_out = stage1_out * 2;
		let stage3_out = stage2_out * 2;
		ReadHead {
			stage_1_conv: ConvBlockConfig::new(self.input_channels, stage1_out)
				.with_kernel_size(5)
				.with_stride(2)
				.with_padding(2)
				.with_momentum(self.stage_1_momentum)
				.init(device),
			stage_1_max_pooling: MaxPool2dConfig::new([2, 2]).init(),
			stage_2a_conv: ConvBlockConfig::new(stage1_out, stage2_out)
				.with_stride(2)
				.with_momentum(self.stage_2a_momentum)
				.init(device),
			stage_2b_conv: ConvBlockConfig::new(stage2_out, stage2_out)
				.with_momentum(self.stage_2b_momentum)
				.init(device),
			stage_3_conv: ConvBlockConfig::new(stage2_out, stage3_out)
				.with_stride(2)
				.with_momentum(self.stage_3_momentum)
				.init(device),
			global_avg_pool: GlobalAvgPooling2d,
		}
	}
}

/// Ingests image data into latent space.
///
/// Should be created with [`ReadHeadConfig`].
#[derive(Module, Debug)]
pub struct ReadHead<B: Backend> {
	stage_1_conv: ConvBlock<B>,
	stage_1_max_pooling: MaxPool2d,
	stage_2a_conv: ConvBlock<B>,
	stage_2b_conv: ConvBlock<B>,
	stage_3_conv: ConvBlock<B>,
	global_avg_pool: GlobalAvgPooling2d,
}

impl<B: Backend> ReadHead<B> {
	/// Runs the forward pass for this layer module.
	///
	/// # Shapes
	///   - Images `[batch_size, channels, height, width]`
	///   - Output `[batch_size, channels]`
	pub fn forward(
		&self,
		input: Tensor<B, 4>,
	) -> Tensor<B, 2> {
		let x = input; // [batch_size, 1, 640, 480]
		let x = self.stage_1_conv.forward(x); // [batch_size, 8, 320, 240]
		let x = self.stage_1_max_pooling.forward(x); // [batch_size, 8, 160, 120]
		let x = self.stage_2a_conv.forward(x); // [batch_size, 16, 80, 60]
		let x = self.stage_2b_conv.forward(x); // [batch_size, 16, 80, 60]
		let x = self.stage_3_conv.forward(x); // [batch_size, 32, 40, 30]
		// TODO: This is a chokepoint.
		self.global_avg_pool.forward(x) // [batch_size, 32]
	}
}
