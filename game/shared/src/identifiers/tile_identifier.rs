use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct TileIdentifier(u64);

impl From<&String> for TileIdentifier
{
	fn from(value: &String) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}

impl From<&str> for TileIdentifier
{
	fn from(value: &str) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}
