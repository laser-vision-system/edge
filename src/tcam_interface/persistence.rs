use std::{
	collections::HashMap,
	ops::{Deref, DerefMut},
};

use super::{Identifier, Metadata};

/// The internal mapping type.
type __Map = HashMap<Identifier, Metadata>;

/// Persistence data for camera devices.
///
/// This type holds details about all currently and previously connected camera devices.
pub struct Persistence(__Map);

impl Deref for Persistence {
	type Target = __Map;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Persistence {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
