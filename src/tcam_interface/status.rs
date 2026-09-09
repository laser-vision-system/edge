/// The current status of the camera device.
///
/// This is a fieldless enum (meaning none of the variants hold data) where the
/// discriminant is the type [`u8`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
	/// This camera device has not yet been initialised. This should never be
	/// the status outside of the camera detection logic.
	#[default]
	Uninitialised = 0,
	/// This camera device is initialised and ready for use, the stream has been
	/// opened.
	Active = 1,
	/// This camera device is not currently connected.
	Disconnected = 2,
	/// This camera can be initialised, but fails to operate correctly.
	Broken = 3,
	/// This device could not be initialised.
	Invalid = 4,
}
