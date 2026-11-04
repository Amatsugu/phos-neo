use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Reflect)]
pub struct UnitIdentifier(u64);

impl From<&String> for UnitIdentifier
{
	fn from(value: &String) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}

impl From<&str> for UnitIdentifier
{
	fn from(value: &str) -> Self
	{
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);

		return Self(hasher.finish());
	}
}
