//! A unique identifier for a camera device.
//!
//! This will match both the actual camera device, as well as any metadata
//! devices. Additionally, if an identical device is plugged into the same port
//! as a previous device, it will be treated as the same device.
//!
//! Note that the device index is not used. This is because the index is not
//! retained when a device is unplugged. There might be a way to enable this
//! behaviour in v4l, but that is outside the project scope for now.
//!
//! All the necessary information should be in the [`CameraInfo`] struct, but
//! only when retrived on an initialised camera.

use nokhwa::utils::CameraInfo;

/// The `Identifier` type.
///
/// See the [module level documentation](self) for more information.
// `PartialOrd` and `Ord` are not strictly necessary.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
	/// The internal name of the device. On v4l, taken from the "Card type" value.
	name: String,
	/// The internal description of the device. On v4l, taken from the "Driver name" value.
	description: String,
	/// Additional miscellaneous information about the device. On v4l, taken from both the "Bus info" value, and the "Driver version" value.
	misc: String,
}

impl From<CameraInfo> for Identifier {
	fn from(value: CameraInfo) -> Self {
		Self {
			name: value.human_name(),
			description: value.description().to_owned(),
			misc: value.misc().to_owned(),
		}
	}
}
