//! A unique identifier for a camera device.
//!
//! This will match both the actual camera device, as well as any metadata
//! devices. Additionally, if a different, but identical device is plugged into
//! the same port as a previous device, it will be treated as the same device.
//!
//! Note that the device index is not used. This is because the index is not
//! retained when a device is unplugged. There might be a way to enable this
//! behaviour in v4l, but that is outside the project scope for now.
//!
//! All the necessary information is in the [`CameraInfo`] struct.

use std::hash::{DefaultHasher, Hash, Hasher};

use nokhwa::utils::CameraInfo;

/// The `Identifier` type.
///
/// See the [module level documentation](self) for more information.
// `PartialOrd` and `Ord` are not strictly necessary.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
	/// The internal name of the device. On v4l, taken from the "Card type" and "Driver name" values.
	name: String,
	/// The internal description of the device. On v4l, taken from the "Bus info" value.
	description: String,
	/// Additional miscellaneous information about the device. On v4l, taken from the "Driver version" value.
	misc: String,
}

impl Identifier {
	pub fn calculate_hash(&self) -> u64 {
		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}
}

impl From<CameraInfo> for Identifier {
	fn from(value: CameraInfo) -> Self {
		Self {
			name: value.human_name().to_string(),
			description: value.description().to_string(),
			misc: value.misc().to_string(),
		}
	}
}

impl PartialEq<u64> for Identifier {
	fn eq(&self, other: &u64) -> bool {
		self.calculate_hash().eq(other)
	}
}

impl PartialEq<Identifier> for u64 {
	fn eq(&self, other: &Identifier) -> bool {
		other.calculate_hash().eq(self)
	}
}
