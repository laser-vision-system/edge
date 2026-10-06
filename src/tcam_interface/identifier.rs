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

#![expect(clippy::inline_modules, reason = "Simple unit tests")]

use std::{
	borrow::{Borrow, Cow},
	fmt,
	hash::{DefaultHasher, Hash, Hasher},
};

use nokhwa::utils::CameraInfo;
use server_edge_datatypes::shared::CameraId;

/// The internal mapping type to the [`CameraInfo`] struct.
///
/// You are probably looking for [`Identifier`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentMetaInfo {
	/// The internal name of the device. On v4l, taken from the "Card type" and
	/// "Driver name" values.
	name: String,
	/// The internal description of the device. On v4l, taken from the "Bus
	/// info" value.
	description: String,
	/// Additional miscellaneous information about the device. On v4l, taken
	/// from the "Driver version" value.
	misc: String,
}

impl IdentMetaInfo {
	/// Get the hash for this instance.
	pub fn calculate_hash(&self) -> CameraId {
		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}

	/// See [`Cow`] for more information.
	pub fn lazy_clone(&self) -> Cow<'_, Self> {
		Cow::Borrowed(self)
	}
}

impl fmt::Display for IdentMetaInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {} {}", self.name, self.description, self.misc)
	}
}

impl Hash for IdentMetaInfo {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.name.hash(state);
		self.description.hash(state);
		self.misc.hash(state);
	}
}

impl From<CameraInfo> for IdentMetaInfo {
	fn from(value: CameraInfo) -> Self {
		Self {
			name: value.human_name().to_owned(),
			description: value.description().to_owned(),
			misc: value.misc().to_owned(),
		}
	}
}

impl PartialEq<CameraId> for IdentMetaInfo {
	fn eq(&self, other: &CameraId) -> bool {
		self.calculate_hash().eq(other)
	}
}

impl PartialEq<IdentMetaInfo> for CameraId {
	fn eq(&self, other: &IdentMetaInfo) -> bool {
		other.calculate_hash().eq(self)
	}
}

impl AsRef<String> for IdentMetaInfo {
	fn as_ref(&self) -> &String { &self.name }
}

/// The `Identifier` type.
///
/// The `meta_info` field is not used when performing [`Hash`], [`PartialEq`],
/// [`PartialOrd`], and [`Ord`].
///
/// See the [module level documentation](self) for more information.
#[derive(Clone, Debug)]
pub struct Identifier {
	hashed_id: CameraId,
	meta_info: IdentMetaInfo,
}

impl AsRef<CameraId> for Identifier {
	fn as_ref(&self) -> &CameraId {
		&self.hashed_id
	}
}

impl AsRef<IdentMetaInfo> for Identifier {
	fn as_ref(&self) -> &IdentMetaInfo { &self.meta_info }
}

impl Identifier {
	/// See [`Cow`] for more information.
	pub fn lazy_clone(&self) -> Cow<'_, Self> {
		Cow::Borrowed(self)
	}
}

impl Borrow<IdentMetaInfo> for Identifier {
	fn borrow(&self) -> &IdentMetaInfo {
		&self.meta_info
	}
}

impl Borrow<CameraId> for Identifier {
	fn borrow(&self) -> &CameraId {
		&self.hashed_id
	}
}

impl fmt::Display for Identifier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.hashed_id, self.meta_info)
	}
}

// Assert to the compiler that this type can be equal to itself.
impl Eq for Identifier {}

impl Hash for Identifier {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.hashed_id.hash(state);
	}
}

impl From<IdentMetaInfo> for Identifier {
	/// If you are doing this conversion directly, you are likely doing
	/// something wrong.
	fn from(value: IdentMetaInfo) -> Self {
		Self {
			hashed_id: value.calculate_hash(),
			meta_info: value,
		}
	}
}

impl From<CameraInfo> for Identifier {
	fn from(value: CameraInfo) -> Self {
		let meta = IdentMetaInfo::from(value);
		Self::from(meta)
	}
}

impl From<Identifier> for CameraId {
	fn from(value: Identifier) -> Self {
		value.hashed_id
	}
}

impl PartialEq for Identifier {
	fn eq(&self, other: &Self) -> bool {
		self.hashed_id.eq(&other.hashed_id)
	}
}

impl PartialOrd for Identifier {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Identifier {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.hashed_id.cmp(&other.hashed_id)
	}
}

#[cfg(test)]
pub mod common {
	use super::IdentMetaInfo;

	impl IdentMetaInfo {
		pub fn new(
			name: impl Into<String>,
			description: impl Into<String>,
			misc: impl Into<String>,
		) -> Self {
			IdentMetaInfo {
				name: name.into(),
				description: description.into(),
				misc: misc.into(),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn identmetainfo_eq() {
		let first = IdentMetaInfo::new("name", "description", "misc");
		let second = first.clone();
		let third = IdentMetaInfo::new("nope", "nuh uh", "hell no");

		assert_eq!(first, second);
		assert_ne!(first, third);
		assert_ne!(second, third);
	}

	#[test]
	fn identifier_eq() {
		let first =
			Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let second = first.clone();
		let third =
			Identifier::from(IdentMetaInfo::new("nope", "nuh uh", "hell no"));

		assert_eq!(first, second);
		assert_ne!(first, third);
		assert_ne!(second, third);
	}

	#[test]
	fn identifier_ord() {
		let first =
			Identifier::from(IdentMetaInfo::new("name", "description", "misc"));
		let second = first.clone();
		let third =
			Identifier::from(IdentMetaInfo::new("nope", "nuh uh", "hell no"));

		assert_eq!(first.partial_cmp(&second), Some(first.cmp(&second)));
		assert_eq!(first.cmp(&third), second.cmp(&third));
		assert_ne!(first.cmp(&second), first.cmp(&third));
	}
}
