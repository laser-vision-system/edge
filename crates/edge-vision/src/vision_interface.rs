use std::fmt;

use crate::{
	batch_item::BatchItem,
	vision_state::VisionState,
};

/// A machine vision laser intensity measuring capable object.
///
/// Implement on a type to declare it as capable of performing machine vision
/// intensity measurement.
pub trait VisionInterface: fmt::Debug + fmt::Display {
	/// Process all frames in a single batch, setting the return value in-place.
	fn process(
		&mut self,
		batch: &mut [BatchItem],
	);

	/// Get a new [`VisionState`] for this technique.
	fn new_state(&self) -> VisionState;

	/// Make sure the batch item has a valid [`VisionState`].
	fn validate_batch(
		&self,
		batch: &mut BatchItem,
	) {
		if !batch.state.is_valid() {
			*batch.state = self.new_state();
		}
	}
}
