//! Provides the [`GlobalAvgPooling2d`] layer module.
//!
//! Functionally similar to the one provided by Keras in Python.

use burn::{
	module::{
		Content,
		DisplaySettings,
		ModuleDisplay,
	},
	prelude::*,
	tensor::module::adaptive_avg_pool2d,
};

/// Global average pooling operation for 2D data.
///
/// Collapses the remaining spatial information into the channels via averaging.
///
/// Does not have any configuration options.
#[derive(Module, Debug)]
#[module(custom_display)]
pub struct GlobalAvgPooling2d;

impl GlobalAvgPooling2d {
	/// Applies the forward pass on the input tensor.
	///
	/// See [`adaptive_avg_pool2d`]
	/// for more information.
	///
	/// # Shapes
	///
	/// - input: `[batch_size, channels, height_in, width_in]`
	/// - output: `[batch_size, channels]`
	#[expect(
		clippy::unused_self,
		reason = "Needed to be included in the model structure"
	)]
	// TODO: Try use associated function instead.
	pub fn forward(
		&self,
		input: Tensor<4>,
	) -> Tensor<2> {
		let [batch_size, channels, ..] = input.dims();
		let x = adaptive_avg_pool2d(input, [1, 1]);
		x.reshape([batch_size, channels])
	}
}

impl ModuleDisplay for GlobalAvgPooling2d {
	fn custom_settings(&self) -> Option<DisplaySettings> {
		DisplaySettings::new()
			.with_new_line_after_attribute(false)
			.optional()
	}

	fn custom_content(
		&self,
		content: Content,
	) -> Option<Content> {
		content.optional()
	}
}
