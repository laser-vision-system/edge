/// The current status of the camera device.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Status {
	/// This device could not be initialised.
	Invalid = -1,
	/// This camera device has just been created, but isn't ready for use.
	#[default]
	Initialised = 0,
	/// This camera device is ready for use, the stream has been opened.
	Active = 1,
	/// This camera device is not currently connected.
	Disconnected = 2,
	/// This camera can be initialised, but fails to operate correctly.
	Broken = 3,
}
