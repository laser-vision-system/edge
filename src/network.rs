//! Shared network utilities.

use serde::de::DeserializeOwned;

use crate::error::{EdgeError, Result};

pub fn unpack<T>(data: &str) -> Result<T>
where
	T: DeserializeOwned,
{
	let data_unchecked: serde_json::Result<T> = serde_json::from_str(data);
	data_unchecked.map_err(|_| EdgeError::WebSocketUnpack)
}
